extern crate alloc;
use core::{
  alloc::Layout,
  fmt,
};

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
  _buffer: Slice,
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
      _buffer: buffer,
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

  pub fn flush(&mut self) -> Result<(), FileError> {
    let rc = unsafe { libc::fflush(self.handle) };
    if rc != 0 {
      return Err(FileError::WriteFailed);
    }
    Ok(())
  }
}

impl fmt::Write for File {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write(s.as_bytes()).map_err(|_| fmt::Error)?;
    Ok(())
  }
}

macro_rules! io_file {
  ($name:ident, $mode:expr, $fd:expr) => {
    paste::paste! {
      #[thread_local]
      static [<STD_ $name:upper>]:
        core::cell::UnsafeCell<core::mem::MaybeUninit<File>> =
        core::cell::UnsafeCell::new(core::mem::MaybeUninit::uninit());

      #[thread_local]
      static [<STD_ $name:upper _INIT>]:
        core::cell::UnsafeCell<bool> =
        core::cell::UnsafeCell::new(false);

      pub fn $name() -> &'static mut File {
        unsafe {
          if !*[<STD_ $name:upper _INIT>].get() {
            let dupped = libc::dup($fd);
            if dupped < 0 {
              panic!("Failed to duplicate file descriptor for {}", stringify!($name));
            }

            let file = File::new_raw(dupped, $mode, FILE_BUFFER).unwrap();
            (*[<STD_ $name:upper>].get()).write(file);
            *[<STD_ $name:upper _INIT>].get() = true;
          }

          &mut *(*[<STD_ $name:upper>].get()).as_mut_ptr()
        }
      }
    }
  };
}

io_file!(stdin, FileMode::Read, libc::STDIN_FILENO);
io_file!(stdout, FileMode::Write, libc::STDOUT_FILENO);
io_file!(stderr, FileMode::Write, libc::STDERR_FILENO);

#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => {{
    use core::fmt::Write;
    let _ = write!($crate::io::stdout(), $($arg)*);
    let _ = $crate::io::stdout().flush();
  }};
}

#[macro_export]
macro_rules! println {
  () => { $crate::print!("\n") };
  ($($arg:tt)*) => {{
    use core::fmt::Write;
    let _ = writeln!($crate::io::stdout(), $($arg)*);
    let _ = $crate::io::stdout().flush();
  }};
}

#[macro_export]
macro_rules! eprint {
  ($($arg:tt)*) => {{
    use core::fmt::Write;
    let _ = write!($crate::io::stderr(), $($arg)*);
    let _ = $crate::io::stderr().flush();
  }};
}

#[macro_export]
macro_rules! eprintln {
  () => { $crate::eprint!("\n") };
  ($($arg:tt)*) => {{
    use core::fmt::Write;
    let _ = writeln!($crate::io::stderr(), $($arg)*);
    let _ = $crate::io::stderr().flush();
  }};
}
