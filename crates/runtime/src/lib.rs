//! A simple wrapper around the [`holochain`](https://docs.rs/holochain/latest/holochain/) create, with functions to expose interacting with the [`holochain::Conductor`](https://docs.rs/holochain/latest/holochain/conductor/index.html) directly.
//! 
//! It is intended to be a general-purpose wrapper for holochain runtime. 
//! 
//! Additionally, some features are included to fulfil its use as an Android runtime. These may or may not be be useful for other contexts. These are:
//! 
//! - Autostart management: tracking if the runtime should be started on system launch. This logic is located in ['autostart.rs']('./src/autostart.rs').
//! - App Client Authorization: tracking which end-user android apps (specified by a package identifier) are allowed to call specified holochain apps. This logic is located in ['authorization.rs'](./src/authorization.rs).

mod runtime;
pub use runtime::*;

mod error;
pub use error::*;

mod config;
pub use config::*;

mod utils;
pub use utils::*;

mod types;
pub use types::*;

mod authorization;
pub use authorization::*;

mod autostart;
pub use autostart::*;

mod persisted;
pub use persisted::*;
