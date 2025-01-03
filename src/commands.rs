use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::M3Ext;
use crate::Result;

#[command]
#[cfg(mobile)]
pub(crate) async fn colors<R: Runtime>(app: AppHandle<R>) -> Result<ColorScheme> {
    app.m3().colors()
}

#[command]
#[cfg(mobile)]
pub(crate) async fn offsets<R: Runtime>(app: AppHandle<R>) -> Result<OffsetsScheme> {
    app.m3().offsets()
}

#[command]
#[cfg(desktop)]
pub(crate) async fn colors<R: Runtime>(app: AppHandle<R>) -> Result<ColorSchemeError> {
    app.m3().colors()
}

#[command]
#[cfg(desktop)]
pub(crate) async fn offsets<R: Runtime>(app: AppHandle<R>) -> Result<OffsetsSchemeError> {
    app.m3().offsets()
}
