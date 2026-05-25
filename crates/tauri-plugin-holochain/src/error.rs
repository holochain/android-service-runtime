use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The conductor has not finished starting yet. Wait for the
    /// `holochain://ready` event before calling `holochain()`.
    #[error("the holochain conductor is not ready yet")]
    NotReady,

    #[error(transparent)]
    Runtime(#[from] holochain_conductor_runtime::RuntimeError),

    #[error(transparent)]
    Tauri(#[from] tauri::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
