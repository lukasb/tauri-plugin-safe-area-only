use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<SafeAreaOnly<R>> {
  Ok(SafeAreaOnly(app.clone()))
}

/// Access to the safe-area-only APIs.
pub struct SafeAreaOnly<R: Runtime>(AppHandle<R>);

impl<R: Runtime> SafeAreaOnly<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
