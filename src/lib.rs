use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;

pub use error::{Error, Result};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("power-manager")
        .invoke_handler(tauri::generate_handler![
            commands::force_logout,
            commands::force_reboot,
            commands::force_shutdown,
            commands::hibernate,
            commands::logout,
            commands::reboot,
            commands::shutdown,
            commands::sleep,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let power_manager = mobile::init(app, api)?;
            #[cfg(desktop)]
            let power_manager = desktop::init(app, api)?;
            app.manage(power_manager);
            Ok(())
        })
        .build()
}
