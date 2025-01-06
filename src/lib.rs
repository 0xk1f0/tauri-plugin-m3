use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::M3;
#[cfg(mobile)]
use mobile::M3;

pub trait M3Ext<R: Runtime> {
    fn m3(&self) -> &M3<R>;
}

impl<R: Runtime, T: Manager<R>> crate::M3Ext<R> for T {
    fn m3(&self) -> &M3<R> {
        self.state::<M3<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("m3")
        .invoke_handler(tauri::generate_handler![
            commands::colors,
            commands::insets
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let m3 = mobile::init(app, api)?;
            #[cfg(desktop)]
            let m3 = desktop::init(app, api)?;
            app.manage(m3);
            Ok(())
        })
        .build()
}
