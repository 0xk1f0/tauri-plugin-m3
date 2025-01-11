use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<M3<R>> {
    Ok(M3(app.clone()))
}

pub struct M3<R: Runtime>(AppHandle<R>);

impl<R: Runtime> M3<R> {
    pub fn colors(&self, _theme: String) -> crate::Result<M3Error> {
        Ok(M3Error {
            error: Some("MaterialYou not supported on this device!".to_string()),
        })
    }
    pub fn insets(&self) -> crate::Result<M3Error> {
        Ok(M3Error {
            error: Some("Insets are not supported on this device!".to_string()),
        })
    }
    pub fn bar_color(&self, _color: String) -> crate::Result<M3Error> {
        Ok(M3Error {
            error: Some("Bar Color not supported on this device!".to_string()),
        })
    }
}
