use crate::error::RuntimeResultFfi;
use holochain_conductor_runtime::AutostartConfigManager;

#[derive(hc_uniffi::Object, Clone)]
pub struct AutostartConfigManagerFfi(AutostartConfigManager);

#[hc_uniffi::export]
impl AutostartConfigManagerFfi {
    #[hc_uniffi::constructor]
    pub fn new(data_root_path: String) -> RuntimeResultFfi<Self> {
        Ok(Self(AutostartConfigManager::new(data_root_path.into())?))
    }

    pub fn enable(&self) -> RuntimeResultFfi<()> {
        Ok(self.0.enable()?)
    }

    pub fn disable(&self) -> RuntimeResultFfi<()> {
        Ok(self.0.disable()?)
    }

    pub fn is_enabled(&self) -> RuntimeResultFfi<bool> {
        Ok(self.0.is_enabled()?)
    }
}
