#![no_std]

#[cfg(windows)]
use core::marker::PhantomData;

#[cfg(windows)]
compile_error!("Windows platform is not supported yet");

mod unix;

pub struct MutexGuard<'m> {
  #[cfg(not(windows))]
  _guard: unix::MutexGuard<'m>,
  #[cfg(windows)]
  _marker: PhantomData<&'m ()>,
}

pub struct Mutex {
  #[cfg(not(windows))]
  inner: unix::Mutex,
}

impl Mutex {
  pub fn new() -> Self {
    #[cfg(not(windows))]
    {
      Self {
        inner: unix::Mutex::new(),
      }
    }

    #[cfg(windows)]
    {
      unimplemented!()
    }
  }

  pub fn lock<'m>(&'m self) -> MutexGuard<'m> {
    #[cfg(not(windows))]
    {
      MutexGuard {
        _guard: self.inner.lock(),
      }
    }

    #[cfg(windows)]
    {
      unimplemented!()
    }
  }
}
