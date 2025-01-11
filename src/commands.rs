use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::M3Ext;
use crate::Result;

#[command]
#[cfg(mobile)]
pub(crate) async fn colors<R: Runtime>(app: AppHandle<R>, theme: String) -> Result<ColorScheme> {
    app.m3().colors(theme)
}

#[command]
#[cfg(mobile)]
pub(crate) async fn insets<R: Runtime>(app: AppHandle<R>) -> Result<InsetsScheme> {
    app.m3().insets()
}

#[command]
#[cfg(mobile)]
pub(crate) async fn bar_color<R: Runtime>(app: AppHandle<R>, color: String) -> Result<BarColorScheme> {
    app.m3().bar_color(color)
}

#[command]
#[cfg(desktop)]
pub(crate) async fn colors<R: Runtime>(app: AppHandle<R>, theme: String) -> Result<M3Error> {
    app.m3().colors(theme)
}

#[command]
#[cfg(desktop)]
pub(crate) async fn insets<R: Runtime>(app: AppHandle<R>) -> Result<M3Error> {
    app.m3().insets()
}

#[command]
#[cfg(desktop)]
pub(crate) async fn bar_color<R: Runtime>(app: AppHandle<R>, color: String) -> Result<M3Error> {
    app.m3().bar_color(color)
}
