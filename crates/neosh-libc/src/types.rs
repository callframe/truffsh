use getset::CloneGetters;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, CloneGetters)]
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
