use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.m3";

pub fn init<R: Runtime, C: DeserializeOwned>(
    #[cfg(target_os = "android")]
    _app: &AppHandle<R>,
    #[cfg(target_os = "ios")]
    app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<M3<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "M3Plugin")?;
    #[cfg(target_os = "ios")]
    let handle = app.clone();
    Ok(M3(handle))
}

#[cfg(target_os = "android")]
pub struct M3<R: Runtime>(PluginHandle<R>);
#[cfg(target_os = "ios")]
pub struct M3<R: Runtime>(AppHandle<R>);

#[cfg(target_os = "android")]
impl<R: Runtime> M3<R> {
    pub fn colors(&self, theme: String) -> crate::Result<ColorScheme> {
        self.0
            .run_mobile_plugin("colors", ColorSchemePayload { theme })
            .map_err(Into::into)
    }
    pub fn insets(&self) -> crate::Result<InsetsScheme> {
        self.0.run_mobile_plugin("insets", "").map_err(Into::into)
    }
    pub fn bar_color(&self, color: String) -> crate::Result<BarColorScheme> {
        self.0
            .run_mobile_plugin("barColor", BarColorPayload { color })
            .map_err(Into::into)
    }
}
#[cfg(target_os = "ios")]
impl<R: Runtime> M3<R> {
    pub fn colors(&self, _theme: String) -> crate::Result<M3Error> {
        Ok(M3Error {
            error: Some("M3 not supported on iOS!".to_string()),
        })
    }
    pub fn insets(&self) -> crate::Result<M3Error> {
        Ok(M3Error {
            error: Some("M3 not supported on iOS!".to_string()),
        })
    }
    pub fn bar_color(&self, _color: String) -> crate::Result<M3Error> {
        Ok(M3Error {
            error: Some("M3 not supported on iOS!".to_string()),
        })
    }
}
