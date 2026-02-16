#![no_std]
#![no_main]

use neo_ffi::add;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
  add(2, 3)
}
