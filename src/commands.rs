use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::M3Ext;
use crate::Result;

#[command]
pub(crate) async fn colors<R: Runtime>(
    app: AppHandle<R>,
) -> Result<ColorScheme> {
    app.m3().colors()
}
