use crate::error::RuntimeResultFfi;
use holochain_conductor_runtime::{AutostartConfigManager, RuntimeNetworkConfig};
use holochain_conductor_runtime_types_ffi::RuntimeNetworkConfigFfi;
use url2::{Url2, Url2Result};

#[derive(uniffi::Object, Clone)]
pub struct AutostartConfigManagerFfi(AutostartConfigManager);

#[uniffi::export]
impl AutostartConfigManagerFfi {
    #[uniffi::constructor]
    pub fn new(data_root_path: String) -> RuntimeResultFfi<Self> {
        Ok(Self(AutostartConfigManager::new(data_root_path.into())?))
    }

    pub fn enable(&self, config: RuntimeNetworkConfigFfi) -> RuntimeResultFfi<()> {
        Ok(self.0.enable(RuntimeNetworkConfig {
            bootstrap_url: Url2::try_parse(config.bootstrap_url)?,
            signal_url: Url2::try_parse(config.signal_url)?,
            ice_urls: config
                .ice_urls
                .into_iter()
                .map(Url2::try_parse)
                .collect::<Url2Result<Vec<Url2>>>()?,
        })?)
    }

    pub fn disable(&self) -> RuntimeResultFfi<()> {
        Ok(self.0.disable()?)
    }

    pub fn get_enabled_config(&self) -> RuntimeResultFfi<Option<RuntimeNetworkConfigFfi>> {
        Ok(self
            .0
            .get_enabled_config()?
            .map(|c| RuntimeNetworkConfigFfi {
                bootstrap_url: c.bootstrap_url.into(),
                signal_url: c.signal_url.into(),
                ice_urls: c.ice_urls.into_iter().map(|u| u.into()).collect(),
            }))
    }
}
