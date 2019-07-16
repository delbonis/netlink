// This crate doesn't work on not-linux, so don't do anything.
#[cfg(not(target_os = "linux"))]
compile_error!("netlink is only supported on Linux");

#[cfg(target_os = "linux")]
mod protocols;

#[cfg(target_os = "linux")]
pub use self::protocols::*;

#[cfg(target_os = "linux")]
mod sys;

#[cfg(target_os = "linux")]
pub mod constants;

#[cfg(target_os = "linux")]
pub use self::sys::*;

#[cfg(all(target_os = "linux", feature = "mio_support"))]
extern crate mio as mio_crate;

#[cfg(all(target_os = "linux", feature = "mio_support"))]
mod mio;

#[cfg(all(target_os = "linux", feature = "tokio_support"))]
#[macro_use]
extern crate log;

#[cfg(all(target_os = "linux", feature = "tokio_support"))]
#[macro_use]
extern crate futures;

#[cfg(all(target_os = "linux", feature = "tokio_support"))]
mod tokio;

#[cfg(all(target_os = "linux", feature = "tokio_support"))]
pub use self::tokio::*;
