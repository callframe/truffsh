#![no_std]

#[cfg(windows)]
compile_error!("Windows is not supported yet");

use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct MutexGuard<'m> {
  mutex: &'m Mutex,
}

impl<'m> Drop for MutexGuard<'m> {
  fn drop(&mut self) {
    self.mutex.unlock();
  }
}

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

  pub fn lock<'m>(&'m self) -> MutexGuard<'m> {
    unsafe {
      libc::pthread_mutex_lock(self.get_mutex_ptr());
    }

    MutexGuard::builder().mutex(self).build()
  }

  fn unlock(&self) {
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
