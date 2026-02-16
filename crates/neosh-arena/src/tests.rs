use super::*;
use core::alloc::Layout;

#[test]
fn test_basic_allocation() {
  let arena = Arena::<1024, 1>::new().unwrap();
  let layout = Layout::from_size_align(16, 1).unwrap();
  let ptr = arena.allocate(layout);
  assert!(!ptr.is_null());

  // Write to and read from allocated memory
  unsafe {
    *ptr = 42;
    assert_eq!(*ptr, 42);
  }
}

#[test]
fn test_multiple_allocations_same_chunk() {
  let arena = Arena::<1024, 1>::new().unwrap();
  let layout = Layout::from_size_align(32, 1).unwrap();

  let ptr1 = arena.allocate(layout);
  let ptr2 = arena.allocate(layout);
  let ptr3 = arena.allocate(layout);

  assert!(!ptr1.is_null());
  assert!(!ptr2.is_null());
  assert!(!ptr3.is_null());

  // Ensure they are distinct and in order
  assert!(ptr1 < ptr2);
  assert!(ptr2 < ptr3);
}

#[test]
fn test_chunk_full_creates_new() {
  let arena = Arena::<64, 1>::new().unwrap(); // Small chunk
  let layout = Layout::from_size_align(32, 1).unwrap();

  // First allocation
  let _ptr1 = arena.allocate(layout);

  // Second allocation should fit
  let _ptr2 = arena.allocate(layout);

  // Third allocation should create new chunk
  let _ptr3 = arena.allocate(layout);
}

#[test]
fn test_zero_sized_allocation() {
  let arena = Arena::<1024, 1>::new().unwrap();
  let layout = Layout::from_size_align(0, 1).unwrap();
  let result = arena.try_allocate(layout);
  assert!(result.is_err());
}

#[test]
fn test_out_of_memory_chunk() {
  let arena = Arena::<64, 1>::new().unwrap();
  let layout = Layout::from_size_align(128, 1).unwrap(); // Larger than chunk
  let result = arena.try_allocate(layout);
  assert!(result.is_err());
}
