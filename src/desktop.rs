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
    pub fn colors(&self) -> crate::Result<ColorSchemeError> {
        Ok(ColorSchemeError {
            error: Some("MaterialYou not supported on this device!".to_string())
        })
    }
}
