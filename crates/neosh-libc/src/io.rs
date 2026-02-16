use core::alloc::Layout;

extern crate alloc;
use crate::types::Slice;
use alloc::alloc::alloc_zeroed;

pub trait Io {
  fn write(&self, offset: usize, data: &[u8]) -> bool;
}

#[repr(u8)]
enum IoBufferMode {
  Read,
  Write,
}

struct IoBuffer<'rw, RW>
where
  RW: Io,
{
  backing: Slice,
  head: usize,
  tail: usize,
  mode: IoBufferMode,
  io: &'rw RW,
}

impl<'rw, RW> IoBuffer<'rw, RW>
where
  RW: Io,
{
  fn new(io: &'rw RW, mode: IoBufferMode, size: usize) -> Self {
    let buffer_layout = Layout::from_size_align(size, 1).unwrap();
    let buffer = unsafe { alloc_zeroed(buffer_layout) };
    let backing = Slice::builder().ptr(buffer).len(size).build();

    Self {
      backing,
      head: 0,
      tail: 0,
      mode,
      io,
    }
  }
}
