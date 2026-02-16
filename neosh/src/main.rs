use core::alloc::Layout;

use libc::fprintf;
use mimalloc::MiMalloc;
use neosh_arena::Arena;

#[global_allocator]
static ALLOC: MiMalloc = MiMalloc;

struct Point {
  x: i32,
  y: i32,
}

unsafe extern "C" {
  static stdout: *mut libc::FILE;
}

fn main() {
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
}
