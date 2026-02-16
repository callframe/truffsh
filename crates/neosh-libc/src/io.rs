const WRITE_MODE: *const libc::c_char = c"w".as_ptr();
const READ_MODE: *const libc::c_char = c"r".as_ptr();

macro_rules! io_handle {
  ($name:ident, $fd:expr, $mode:expr) => {
    paste::paste! {
        #[thread_local]
        static [<$name:upper>]: core::cell::UnsafeCell<*mut libc::FILE> = core::cell::UnsafeCell::new(core::ptr::null_mut());

        pub fn [<get_ $name>]() -> *mut libc::FILE {
          let handle_ptr = unsafe { *[<$name:upper>].get() };
          if handle_ptr.is_null() {
            let file_ptr = unsafe { libc::fdopen($fd, $mode as *const i8) };
            unsafe { *[<$name:upper>].get() = file_ptr };
          }

          unsafe { *[<$name:upper>].get() }
        }
    }
  };
}

io_handle!(stdout, libc::STDOUT_FILENO, WRITE_MODE);
io_handle!(stderr, libc::STDERR_FILENO, WRITE_MODE);
io_handle!(stdin, libc::STDIN_FILENO, READ_MODE);
