#![cfg(not(windows))]

use core::{
  mem::MaybeUninit,
  ptr,
};

pub struct MutexGuard<'m> {
  mutex: &'m Mutex,
}

impl<'m> Drop for MutexGuard<'m> {
  fn drop(&mut self) {
    self.mutex.unlock();
  }
}

pub struct Mutex {
  inner: libc::pthread_mutex_t,
}

impl Mutex {
  pub fn new() -> Self {
    let inner = unsafe {
      let mut inner = MaybeUninit::uninit();
      libc::pthread_mutex_init(inner.as_mut_ptr(), ptr::null());
      inner.assume_init()
    };

    Self { inner }
  }

  fn get_mutex(&self) -> *mut libc::pthread_mutex_t {
    &self.inner as *const _ as *mut _
  }

  pub fn lock<'m>(&'m self) -> MutexGuard<'m> {
    unsafe {
      libc::pthread_mutex_lock(self.get_mutex());
    }

    MutexGuard { mutex: self }
  }

  fn unlock(&self) {
    unsafe {
      libc::pthread_mutex_unlock(self.get_mutex());
    }
  }
}

impl Drop for Mutex {
  fn drop(&mut self) {
    unsafe {
      libc::pthread_mutex_destroy(self.get_mutex());
    }
  }
}
