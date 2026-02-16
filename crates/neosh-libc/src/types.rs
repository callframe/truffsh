extern crate alloc;
use alloc::vec::Vec;

use getset::CloneGetters;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, CloneGetters, Clone)]
pub struct Slice {
  #[getset(get_clone = "pub")]
  ptr: *mut u8,
  #[getset(get_clone = "pub")]
  len: usize,
}

impl AsRef<[u8]> for Slice {
  fn as_ref(&self) -> &[u8] {
    unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
  }
}

impl AsMut<[u8]> for Slice {
  fn as_mut(&mut self) -> &mut [u8] {
    unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
  }
}

pub fn cstr(s: &str) -> Vec<u8> {
  let mut cstr = Vec::with_capacity(s.len() + 1);
  cstr.extend_from_slice(s.as_bytes());
  cstr.push(0);
  cstr
}
