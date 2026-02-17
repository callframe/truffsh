use core::{
  fmt::Write,
  panic::PanicInfo,
};

use neosh_libc::io::stderr;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  let stderr = stderr();

  if let Some(location) = info.location() {
    let _ = write!(
      stderr,
      "thread 'main' panicked at {}:{}:{}\n  {}",
      location.file(),
      location.line(),
      location.column(),
      info.message(),
    );
  } else {
    let _ = write!(stderr, "thread 'main' panicked: {}", info.message());
  }

  let _ = writeln!(stderr);
  let _ = stderr.flush();

  unsafe { libc::abort() };
}
