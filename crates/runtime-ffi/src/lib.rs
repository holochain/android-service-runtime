uniffi::setup_scaffolding!();

extern crate android_logger;
extern crate log;

mod error;
mod multi_thread;
mod runtime;

pub use runtime::*;
