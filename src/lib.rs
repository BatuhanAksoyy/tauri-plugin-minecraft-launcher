use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::MinecraftLauncher;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the minecraft-launcher APIs.
pub trait MinecraftLauncherExt<R: Runtime> {
    fn minecraft_launcher(&self) -> &MinecraftLauncher<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MinecraftLauncherExt<R> for T {
    fn minecraft_launcher(&self) -> &MinecraftLauncher<R> {
        self.state::<MinecraftLauncher<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("minecraft-launcher")
        .invoke_handler(tauri::generate_handler![
            commands::get_vanilla_versions,
            commands::get_fabric_versions,
            commands::get_fabric_loaders,
            commands::get_quilt_versions,
            commands::get_quilt_loaders,
            commands::get_forge_metadata
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            unimplemented!();
            #[cfg(desktop)]
            let minecraft_launcher = desktop::init(app, api)?;
            app.manage(minecraft_launcher);
            Ok(())
        })
        .build()
}
