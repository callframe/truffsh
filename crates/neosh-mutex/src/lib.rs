#![no_std]

pub struct Mutex {
  #[cfg(unix)]
  inner: libc::pthread_mutex_t,
}

#[cfg(unix)]
impl Mutex {
  pub fn new() -> Self {
    use core::{
      mem::MaybeUninit,
      ptr,
    };

    unsafe {
      let mut inner = MaybeUninit::uninit();
      libc::pthread_mutex_init(inner.as_mut_ptr(), ptr::null());

      Self {
        inner: inner.assume_init(),
      }
    }
  }

  fn get_mutex_ptr(&self) -> *mut libc::pthread_mutex_t {
    &self.inner as *const _ as *mut _
  }

  pub fn lock(&self) {
    unsafe {
      libc::pthread_mutex_lock(self.get_mutex_ptr());
    }
  }

  pub fn unlock(&self) {
    unsafe {
      libc::pthread_mutex_unlock(self.get_mutex_ptr());
    }
  }
}

#[cfg(unix)]
impl Drop for Mutex {
  fn drop(&mut self) {
    unsafe {
      libc::pthread_mutex_destroy(&mut self.inner);
    }
  }
}
