use crate::error::RuntimeConfigErrorFfi;
use holochain_conductor_runtime::RuntimeConfig;
use url2::Url2;

#[derive(uniffi::Record, Clone, Debug)]
pub struct RuntimeConfigFfi {
    /// Path where conductor data is stored
    pub data_root_path: String,

    /// URL of the bootstrap server
    pub bootstrap_url: String,

    /// URL of the sbd server
    pub signal_url: String,
}

impl TryInto<RuntimeConfig> for RuntimeConfigFfi {
    type Error = RuntimeConfigErrorFfi;
    fn try_into(self) -> Result<RuntimeConfig, Self::Error> {
        Ok(RuntimeConfig {
            data_root_path: self.data_root_path.into(),
            bootstrap_url: Url2::try_parse(self.bootstrap_url)?,
            signal_url: Url2::try_parse(self.signal_url)?,
        })
    }
}
