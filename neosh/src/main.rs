#![no_std]
#![no_main]

mod panic;

use core::{
  alloc::Layout,
  fmt::Display,
};

use mimalloc::MiMalloc;
use neosh_arena::Arena;
use neosh_libc::println;

#[global_allocator]
static ALLOC: MiMalloc = MiMalloc;

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

impl Display for Point {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
  let arena = Arena::default();
  let point = arena.allocate(Layout::new::<Point>()) as *mut Point;
  unsafe {
    *point = Point { x: 10, y: 20 };
  }

  let point = unsafe { &*point };
  println!("{}", point);
  0
}
