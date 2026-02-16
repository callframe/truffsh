#![no_std]

extern crate alloc;

use alloc::{
  boxed::Box,
  vec::Vec,
};
use core::{
  alloc::Layout,
  mem::MaybeUninit,
  sync::atomic::AtomicUsize,
};
use neosh_mutex::Mutex;

#[inline(always)]
fn align_up(addr: usize, align: usize) -> usize {
  (addr + align - 1) & !(align - 1)
}

struct ArenaChunk<const CHUNK_SIZE: usize = 1, const MIN_ALIGN: usize = 1> {
  data: [MaybeUninit<u8>; CHUNK_SIZE],
  used: AtomicUsize,
}

type BoxedArenaChunk<const CHUNK_SIZE: usize = 1, const MIN_ALIGN: usize = 1> =
  Box<ArenaChunk<CHUNK_SIZE, MIN_ALIGN>>;

impl<const CHUNK_SIZE: usize, const MIN_ALIGN: usize> ArenaChunk<CHUNK_SIZE, MIN_ALIGN> {
  pub fn new() -> Self {
    Self {
      data: unsafe { MaybeUninit::uninit().assume_init() },
      used: AtomicUsize::new(0),
    }
  }
}

pub struct Arena<const CHUNK_SIZE: usize = 1, const MIN_ALIGN: usize = 1> {
  chunks: Vec<BoxedArenaChunk<CHUNK_SIZE, MIN_ALIGN>>,
  lock: Mutex,
}

impl<const CHUNK_SIZE: usize, const MIN_ALIGN: usize> Arena<CHUNK_SIZE, MIN_ALIGN> {
  pub fn new() -> Self {
    Self {
      chunks: Vec::new(),
      lock: Mutex::new(),
    }
  }
}
