//! # holochain-conductor-runtime-ffi
//!
//! A simple wrapper around `holochain-conductor-runtime`,
//! with the addition of using [`uniffi`](https://docs.rs/uniffi/latest/uniffi/)
//! to generate FFI functions and Kotlin bindings to them.

uniffi::setup_scaffolding!();

extern crate android_logger;
extern crate log;

mod autostart;
mod error;
mod multi_thread;
mod runtime;

pub use autostart::*;
pub use runtime::*;
