uniffi::setup_scaffolding!();

extern crate log;
extern crate android_logger;

mod runtime;
mod error;
mod config;

pub use runtime::*;
