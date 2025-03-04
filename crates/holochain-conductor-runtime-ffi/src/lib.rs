uniffi::setup_scaffolding!();

#[macro_use]
extern crate log;
extern crate android_logger;

mod config;
mod error;
mod runtime;
mod types;
pub use runtime::*;
