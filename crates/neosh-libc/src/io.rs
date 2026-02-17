extern crate alloc;
use core::alloc::Layout;

use alloc::alloc::{
  alloc_zeroed,
  dealloc,
};

use crate::types::{
  Slice,
  cstr,
};

pub const FILE_BUFFER: usize = 1 << 10;

#[repr(u8)]
#[derive(Clone)]
pub enum FileMode {
  Read,
  Write,
  ReadWrite,
}

impl From<FileMode> for *const libc::c_char {
  fn from(mode: FileMode) -> Self {
    match mode {
      FileMode::Read => c"rb".as_ptr(),
      FileMode::Write => c"wb".as_ptr(),
      FileMode::ReadWrite => c"r+b".as_ptr(),
    }
  }
}

#[repr(u8)]
#[derive(Debug)]
pub enum FileError {
  OpenFailed,
  SeekFailed,
  ReadFailed,
  WriteFailed,
  OutOfMemory,
}

#[repr(u8)]
pub enum FileWhence {
  Start,
  Current,
  End,
}

impl From<FileWhence> for libc::c_int {
  fn from(whence: FileWhence) -> Self {
    match whence {
      FileWhence::Start => libc::SEEK_SET,
      FileWhence::Current => libc::SEEK_CUR,
      FileWhence::End => libc::SEEK_END,
    }
  }
}

pub struct File {
  handle: *mut libc::FILE,
  buffer: Slice,
}

impl File {
  fn buffer_layout(buffer_size: usize) -> Layout {
    Layout::from_size_align(buffer_size, 1).unwrap()
  }

  fn create_buffer(buffer_size: usize) -> Result<Slice, FileError> {
    let buffer_ptr = unsafe { alloc_zeroed(Self::buffer_layout(buffer_size)) };
    if buffer_ptr.is_null() {
      return Err(FileError::OutOfMemory);
    }
    Ok(Slice::builder().ptr(buffer_ptr).len(buffer_size).build())
  }

  fn use_buffer(file: *mut libc::FILE, buffer: &Slice) -> Result<(), FileError> {
    let result =
      unsafe { libc::setvbuf(file, buffer.ptr() as *mut i8, libc::_IOFBF, buffer.len()) };
    if result != 0 {
      return Err(FileError::OpenFailed);
    }
    Ok(())
  }

  fn destroy_buffer(buffer: &Slice) {
    unsafe { dealloc(buffer.ptr(), Self::buffer_layout(buffer.len())) };
  }

  fn from_handle(file: *mut libc::FILE, buffer_size: usize) -> Result<Self, FileError> {
    let buffer = match Self::create_buffer(buffer_size) {
      Ok(buf) => buf,
      Err(e) => {
        unsafe { libc::fclose(file) };
        return Err(e);
      }
    };

    match Self::use_buffer(file, &buffer) {
      Ok(()) => (),
      Err(e) => {
        unsafe { libc::fclose(file) };
        Self::destroy_buffer(&buffer);
        return Err(e);
      }
    }

    let file = Self {
      handle: file,
      buffer,
    };
    Ok(file)
  }

  pub fn new(path: &str, mode: FileMode) -> Result<Self, FileError> {
    let c_path = cstr(path);

    let file = unsafe { libc::fopen(c_path.as_ptr() as *const libc::c_char, mode.clone().into()) };
    if file.is_null() {
      return Err(FileError::OpenFailed);
    }

    Self::from_handle(file, FILE_BUFFER)
  }

  pub unsafe fn new_raw(fd: i32, mode: FileMode, buffer_size: usize) -> Result<Self, FileError> {
    let file = unsafe { libc::fdopen(fd, mode.into()) };
    if file.is_null() {
      return Err(FileError::OpenFailed);
    }

    Self::from_handle(file, buffer_size)
  }

  pub fn seek(&mut self, offset: i64, whence: FileWhence) -> Result<(), FileError> {
    let rc = unsafe { libc::fseek(self.handle, offset as libc::c_long, whence.into()) };
    if rc != 0 {
      return Err(FileError::SeekFailed);
    }
    Ok(())
  }

  pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, FileError> {
    let n = unsafe { libc::fread(buf.as_mut_ptr().cast(), 1, buf.len(), self.handle) };
    if n == 0 && unsafe { libc::ferror(self.handle) } != 0 {
      return Err(FileError::ReadFailed);
    }
    Ok(n)
  }

  pub fn write(&mut self, buf: &[u8]) -> Result<usize, FileError> {
    let n = unsafe { libc::fwrite(buf.as_ptr().cast(), 1, buf.len(), self.handle) };
    if n == 0 && unsafe { libc::ferror(self.handle) } != 0 {
      return Err(FileError::WriteFailed);
    }
    Ok(n)
  }
}

pub trait Read {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, FileError>;
}

pub trait Write {
  fn write(&mut self, buf: &[u8]) -> Result<usize, FileError>;
}

impl Read for File {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, FileError> {
    self.read(buf)
  }
}

impl Write for File {
  fn write(&mut self, buf: &[u8]) -> Result<usize, FileError> {
    self.write(buf)
  }
}

impl Drop for File {
  fn drop(&mut self) {
    unsafe { libc::fclose(self.handle) };
    Self::destroy_buffer(&self.buffer);
  }
}
