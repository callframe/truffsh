#![no_std]
#![no_main]

use core::{
  alloc::Layout,
  panic::PanicInfo,
};

use libc::fprintf;
use mimalloc::MiMalloc;
use neosh_arena::Arena;

#[global_allocator]
static ALLOC: MiMalloc = MiMalloc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

struct Point {
  x: i32,
  y: i32,
}

unsafe extern "C" {
  static stdout: *mut libc::FILE;
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
  let arena = Arena::default();
  let point = arena.allocate(Layout::new::<Point>()) as *mut Point;
  unsafe {
    *point = Point { x: 10, y: 20 };
  }

  unsafe {
    fprintf(
      stdout,
      b"Point: (%d, %d)\n\0".as_ptr() as *const i8,
      (*point).x,
      (*point).y,
    );
  }

  0
}
