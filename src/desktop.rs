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
    pub fn colors(&self) -> crate::Result<ColorScheme> {
        Ok(ColorScheme {
            primary: Some("#000000".to_string()),
            on_primary: Some("#000000".to_string()),
            primary_container: Some("#000000".to_string()),
            on_primary_container: Some("#000000".to_string()),
            inverse_primary: Some("#000000".to_string()),
            secondary: Some("#000000".to_string()),
            on_secondary: Some("#000000".to_string()),
            secondary_container: Some("#000000".to_string()),
            on_secondary_container: Some("#000000".to_string()),
            tertiary: Some("#000000".to_string()),
            on_tertiary: Some("#000000".to_string()),
            tertiary_container: Some("#000000".to_string()),
            on_tertiary_container: Some("#000000".to_string()),
            background: Some("#000000".to_string()),
            on_background: Some("#000000".to_string()),
            surface: Some("#000000".to_string()),
            on_surface: Some("#000000".to_string()),
            surface_variant: Some("#000000".to_string()),
            on_surface_variant: Some("#000000".to_string()),
            inverse_surface: Some("#000000".to_string()),
            inverse_on_surface: Some("#000000".to_string()),
            outline: Some("#000000".to_string()),
        })
    }
}
