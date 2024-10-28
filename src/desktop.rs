use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<PowerManager> {
    Ok(PowerManager)
}

/// Access to the power-manager APIs.
pub struct PowerManager;
