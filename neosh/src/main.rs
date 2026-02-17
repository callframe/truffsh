#![no_std]
#![no_main]

mod panic;

use core::alloc::Layout;

use mimalloc::MiMalloc;
use neosh_arena::Arena;
use neosh_libc::{
  eprintln,
  println,
};

#[global_allocator]
static ALLOC: MiMalloc = MiMalloc;

struct Point {
  x: i32,
  y: i32,
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
  let arena = Arena::default();
  let point = arena.allocate(Layout::new::<Point>()) as *mut Point;
  unsafe {
    *point = Point { x: 10, y: 20 };
  }

  let point = unsafe { &*point };
  println!("Point: ({}, {})", point.x, point.y);
  eprintln!("debug: point allocated at arena");

  let x = 42;
  panic!(
    "unexpected value x={x} for point ({}, {})",
    point.x, point.y
  );

  0
}
