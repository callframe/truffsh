#![no_std]

extern crate alloc;

use alloc::{
  alloc::{
    alloc_zeroed,
    dealloc,
  },
  vec::Vec,
};
use core::{
  alloc::Layout,
  cell::UnsafeCell,
};
use neosh_mutex::Mutex;

pub const CHUNK_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
pub enum ArenaError {
  OutOfMemory,
  TooLarge,
  ZeroSized,
}

#[inline(always)]
fn align_up(addr: usize, align: usize) -> usize {
  (addr + align - 1) & !(align - 1)
}

struct ArenaChunkRange {
  start: usize,
  end: usize,
}

struct ArenaChunk<const CHUNK_SIZE: usize, const MIN_ALIGN: usize> {
  ptr: *mut u8,
  capacity: usize,
  used: usize,
}

impl<const CHUNK_SIZE: usize, const MIN_ALIGN: usize> ArenaChunk<CHUNK_SIZE, MIN_ALIGN> {
  pub fn new() -> Result<Self, ArenaError> {
    let layout = Layout::from_size_align(CHUNK_SIZE, MIN_ALIGN).unwrap();
    let ptr = unsafe { alloc_zeroed(layout) };
    if ptr.is_null() {
      return Err(ArenaError::OutOfMemory);
    }

    let chunk = Self {
      ptr,
      capacity: CHUNK_SIZE,
      used: 0,
    };

    Ok(chunk)
  }

  #[inline(always)]
  fn get_current_addr(&self) -> usize {
    self.ptr as usize + self.used
  }

  #[inline(always)]
  fn get_aligned(&self, layout: Layout) -> ArenaChunkRange {
    let curr = self.get_current_addr();
    let aligned = align_up(curr, layout.align());
    ArenaChunkRange {
      start: aligned,
      end: aligned + layout.size(),
    }
  }

  pub fn allocate(&mut self, layout: Layout) -> Option<*mut u8> {
    assert!(layout.size() > 0);
    assert!(layout.size() <= self.capacity);

    let aligned = self.get_aligned(layout);
    let new_used = aligned.end - self.ptr as usize;
    if new_used > self.capacity {
      return None;
    }

    self.used = new_used;
    Some(aligned.start as *mut u8)
  }
}

impl<const CHUNK_SIZE: usize, const MIN_ALIGN: usize> Drop for ArenaChunk<CHUNK_SIZE, MIN_ALIGN> {
  fn drop(&mut self) {
    unsafe {
      let layout = Layout::from_size_align(CHUNK_SIZE, MIN_ALIGN).unwrap();
      dealloc(self.ptr, layout);
    }
  }
}

pub struct Arena<const CSIZE: usize = CHUNK_SIZE, const MIN_ALIGN: usize = CSIZE> {
  chunks: UnsafeCell<Vec<ArenaChunk<CSIZE, MIN_ALIGN>>>,
  lock: Mutex,
}

impl<const CHUNK_SIZE: usize, const MIN_ALIGN: usize> Arena<CHUNK_SIZE, MIN_ALIGN> {
  pub fn new() -> Result<Self, ArenaError> {
    let chunks = Vec::new();
    let arena = Self {
      chunks: UnsafeCell::new(chunks),
      lock: Mutex::new(),
    };

    Ok(arena)
  }

  unsafe fn get_chunks(&self) -> &mut Vec<ArenaChunk<CHUNK_SIZE, MIN_ALIGN>> {
    unsafe { &mut *self.chunks.get() }
  }

  pub fn try_allocate(&self, layout: Layout) -> Result<*mut u8, ArenaError> {
    if layout.size() == 0 {
      return Err(ArenaError::ZeroSized);
    }

    if layout.size() > CHUNK_SIZE {
      return Err(ArenaError::TooLarge);
    }

    self.lock.lock();
    let chunks = unsafe { self.get_chunks() };

    if let Some(chunk) = chunks.last_mut() {
      if let Some(ptr) = chunk.allocate(layout) {
        self.lock.unlock();
        return Ok(ptr);
      }
    }

    let mut new_chunk = ArenaChunk::<CHUNK_SIZE, MIN_ALIGN>::new()?;
    let ptr = new_chunk.allocate(layout).unwrap();
    chunks.push(new_chunk);

    self.lock.unlock();
    Ok(ptr)
  }

  pub fn allocate(&self, layout: Layout) -> *mut u8 {
    self.try_allocate(layout).unwrap()
  }
}

unsafe impl<const CHUNK_SIZE: usize, const MIN_ALIGN: usize> Sync for Arena<CHUNK_SIZE, MIN_ALIGN> {}

#[cfg(test)]
mod tests;
