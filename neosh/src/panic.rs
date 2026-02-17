use core::panic::PanicInfo;

use neosh_libc::eprintln;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  if let Some(location) = info.location() {
    eprintln!(
      "thread 'main' panicked at {}:{}:{}\n  {}",
      location.file(),
      location.line(),
      location.column(),
      info.message(),
    );
  } else {
    eprintln!("thread 'main' panicked: {}", info.message());
  }

  unsafe { libc::abort() }
}
