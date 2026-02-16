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
use typed_builder::TypedBuilder;

pub const CHUNK_SIZE: usize = 1024 * 1024;
pub const CHUNK_ALIGN: usize = 16;

#[derive(Debug)]
pub enum ArenaError {
  OutOfMemory,
  TooLarge,
  ZeroSized,
  InvalidLayout,
}

#[inline(always)]
fn align_up(addr: usize, align: usize) -> usize {
  (addr + align - 1) & !(align - 1)
}

#[derive(TypedBuilder)]
struct ArenaChunkRange {
  start: usize,
  end: usize,
}

struct ArenaChunk<const CHUNK_SIZE: usize, const MIN_ALIGN: usize> {
  ptr: *mut u8,
  capacity: usize,
  used: usize,
}

impl<const CSIZE: usize, const CALIGN: usize> ArenaChunk<CSIZE, CALIGN> {
  pub fn new() -> Result<Self, ArenaError> {
    let layout = match Layout::from_size_align(CSIZE, CALIGN) {
      Ok(layout) => layout,
      Err(_) => return Err(ArenaError::InvalidLayout),
    };

    let ptr = unsafe { alloc_zeroed(layout) };
    if ptr.is_null() {
      return Err(ArenaError::OutOfMemory);
    }

    let chunk = Self {
      ptr,
      capacity: CSIZE,
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
    let end = aligned + layout.size();

    ArenaChunkRange::builder().start(aligned).end(end).build()
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
      let layout = Layout::from_size_align_unchecked(self.capacity, MIN_ALIGN);
      dealloc(self.ptr, layout);
    }
  }
}

pub struct Arena<const CSIZE: usize = CHUNK_SIZE, const CALIGN: usize = CHUNK_ALIGN> {
  chunks: UnsafeCell<Vec<ArenaChunk<CSIZE, CALIGN>>>,
  lock: Mutex,
}

impl<const CSIZE: usize, const CALIGN: usize> Arena<CSIZE, CALIGN> {
  pub fn new() -> Result<Self, ArenaError> {
    let chunks = Vec::new();
    let arena = Self {
      chunks: UnsafeCell::new(chunks),
      lock: Mutex::new(),
    };

    Ok(arena)
  }

  unsafe fn get_chunks(&self) -> &mut Vec<ArenaChunk<CSIZE, CALIGN>> {
    unsafe { &mut *self.chunks.get() }
  }

  pub fn try_allocate(&self, layout: Layout) -> Result<*mut u8, ArenaError> {
    if layout.size() == 0 {
      return Err(ArenaError::ZeroSized);
    }

    if layout.size() > CSIZE {
      return Err(ArenaError::TooLarge);
    }

    let _guard = self.lock.lock();
    let chunks = unsafe { self.get_chunks() };

    if let Some(chunk) = chunks.last_mut() {
      if let Some(ptr) = chunk.allocate(layout) {
        return Ok(ptr);
      }
    }

    let mut new_chunk = ArenaChunk::<CSIZE, CALIGN>::new()?;
    let ptr = new_chunk.allocate(layout).unwrap();
    chunks.push(new_chunk);
    Ok(ptr)
  }

  pub fn allocate(&self, layout: Layout) -> *mut u8 {
    self.try_allocate(layout).unwrap()
  }
}

impl Arena<CHUNK_SIZE, CHUNK_ALIGN> {
  pub fn default() -> Self {
    Self::new().unwrap()
  }
}

unsafe impl<const CSIZE: usize, const CALIGN: usize> Sync for Arena<CSIZE, CALIGN> {}

#[cfg(test)]
mod tests;
