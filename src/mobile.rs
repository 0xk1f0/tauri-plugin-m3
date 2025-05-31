use serde::de::DeserializeOwned;
use serde::Serialize;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.m3";

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<M3<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "M3Plugin")?;
    #[cfg(target_os = "ios")]
    let handle = app.clone();
    Ok(M3(handle))
}

pub struct M3<R: Runtime>(PluginHandle<R>);

#[derive(Serialize)]
struct ColorSchemePayload {
    theme: String
}

#[derive(Serialize)]
struct BarColorPayload {
    color: String
}

impl<R: Runtime> M3<R> {
    pub fn colors(&self, theme: String) -> crate::Result<ColorScheme> {
        self.0.run_mobile_plugin("colors", ColorSchemePayload { theme }).map_err(Into::into)
    }
    pub fn insets(&self) -> crate::Result<InsetsScheme> {
        self.0.run_mobile_plugin("insets", "").map_err(Into::into)
    }
    pub fn bar_color(&self, color: String) -> crate::Result<BarColorScheme> {
        self.0.run_mobile_plugin("barColor", BarColorPayload { color }).map_err(Into::into)
    }
}
