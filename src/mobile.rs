use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.m3";

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<M3<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "M3Plugin")?;
    Ok(M3(handle))
}

/// Access to the m3 APIs.
pub struct M3<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> M3<R> {
    pub fn colors(&self) -> crate::Result<ColorScheme> {
        self.0
            .run_mobile_plugin("colors", "")
            .map_err(Into::into)
    }
}
