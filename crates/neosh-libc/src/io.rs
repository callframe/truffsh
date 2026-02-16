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
      FileMode::Read => c"r".as_ptr(),
      FileMode::Write => c"w".as_ptr(),
      FileMode::ReadWrite => c"r+".as_ptr(),
    }
  }
}

#[repr(u8)]
#[derive(Debug)]
pub enum FileError {
  OpenFailed,
  ReadFailed,
  WriteFailed,
  OutOfMemory,
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

  pub fn new(path: &str, mode: FileMode) -> Result<Self, FileError> {
    let c_path = cstr(path);

    let file = unsafe { libc::fopen(c_path.as_ptr() as *const libc::c_char, mode.clone().into()) };
    if file.is_null() {
      return Err(FileError::OpenFailed);
    }

    let buffer = Self::create_buffer(FILE_BUFFER)?;
    Self::use_buffer(file, &buffer)?;

    Ok(Self {
      handle: file,
      buffer,
    })
  }

  pub unsafe fn new_raw(
    fd: libc::c_int,
    mode: FileMode,
    buffer_size: usize,
  ) -> Result<Self, FileError> {
    let file = unsafe { libc::fdopen(fd, mode.into()) };
    if file.is_null() {
      return Err(FileError::OpenFailed);
    }

    let buffer = Self::create_buffer(buffer_size)?;
    Self::use_buffer(file, &buffer)?;

    Ok(Self {
      handle: file,
      buffer,
    })
  }
}

impl Drop for File {
  fn drop(&mut self) {
    unsafe { libc::fclose(self.handle) };
    unsafe { dealloc(self.buffer.ptr(), Self::buffer_layout(self.buffer.len())) };
  }
}
