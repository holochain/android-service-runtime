use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The conductor has not finished starting yet. Wait for the
    /// `holochain://ready` event before calling `holochain()`.
    #[error("the holochain conductor is not ready yet")]
    NotReady,

    /// [`crate::HolochainPlugin::start`] was called while the conductor is
    /// already running.
    #[error("the holochain conductor is already started")]
    AlreadyStarted,

    #[error(transparent)]
    Runtime(#[from] holochain_conductor_runtime::RuntimeError),

    #[error(transparent)]
    Tauri(#[from] tauri::Error),

    /// An `app_request` arrived from a window that is not bound to any app.
    /// Windows are bound by [`crate::HolochainPlugin::main_window_builder`].
    #[error("no holochain app is bound to this window")]
    WindowNotBound,

    /// Failed to (de)serialize an App API request or response.
    #[error("serialization error: {0}")]
    Serialization(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
