#![no_std]
#![feature(thread_local)]

#[cfg(windows)]
compile_error!("Windows platform is not supported yet");

pub mod io;
pub mod types;
