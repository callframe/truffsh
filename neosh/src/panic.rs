use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  let msg = info.message();
  println!("PANIC: {}", msg);
  unsafe { libc::abort() };
}
