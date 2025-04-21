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
use desktop::SafeAreaOnly;
#[cfg(mobile)]
use mobile::SafeAreaOnly;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the safe-area-only APIs.
pub trait SafeAreaOnlyExt<R: Runtime> {
  fn safe_area_only(&self) -> &SafeAreaOnly<R>;
}

impl<R: Runtime, T: Manager<R>> crate::SafeAreaOnlyExt<R> for T {
  fn safe_area_only(&self) -> &SafeAreaOnly<R> {
    self.state::<SafeAreaOnly<R>>().inner()
  }
}

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_safe_area_only);

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("safe-area-only")
    .setup(|_app, api| {
      #[cfg(target_os = "ios")]
      api.register_ios_plugin(init_plugin_safe_area_only)?;
      Ok(())
    })
    .build()
}
