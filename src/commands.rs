use tauri::command;

use crate::{Error, Result};

#[command]
pub(crate) fn force_logout() -> Result<()> {
    system_shutdown::force_logout().map_err(|_| Error)
}

#[command]
pub(crate) fn force_reboot() -> Result<()> {
    system_shutdown::force_reboot().map_err(|_| Error)
}

#[command]
pub(crate) fn force_shutdown() -> Result<()> {
    system_shutdown::force_shutdown().map_err(|_| Error)
}

#[command]
pub(crate) fn hibernate() -> Result<()> {
    system_shutdown::hibernate().map_err(|_| Error)
}

#[command]
pub(crate) fn logout() -> Result<()> {
    system_shutdown::logout().map_err(|_| Error)
}

#[command]
pub(crate) fn reboot() -> Result<()> {
    system_shutdown::reboot().map_err(|_| Error)
}

#[command]
pub(crate) fn shutdown() -> Result<()> {
    system_shutdown::shutdown().map_err(|_| Error)
}

#[command]
pub(crate) fn sleep() -> Result<()> {
    system_shutdown::sleep().map_err(|_| Error)
}
