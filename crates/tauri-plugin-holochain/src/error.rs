use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// No conductor boot has finished: none was started, or one is in flight.
    /// Also returned by [`crate::HolochainExt::holochain`] when the plugin is
    /// not registered at all.
    #[error("the holochain conductor is not ready yet")]
    NotReady,

    /// The conductor was started and failed to come up, carrying the same cause
    /// `holochain://setup-failed` reports.
    #[error("the holochain conductor failed to start: {0}")]
    SetupFailed(String),

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
