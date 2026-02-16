#![no_std]
#![no_main]

use core::panic::PanicInfo;
use neo_ffi::unistd::write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
  let message = "Hello, NeoSH!\n";
  let message_c: *const core::ffi::c_void = message.as_ptr() as *const core::ffi::c_void;
  let message_len = message.len();
  unsafe {
    write(1, message_c, message_len);
  }
  0
}
