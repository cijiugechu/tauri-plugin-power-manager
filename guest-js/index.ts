import { invoke } from '@tauri-apps/api/core'

/**
 * Calls the OS-specific function to force to log out the user.
 */
export const forceLogout = async () => {
  await invoke('plugin:power-manager|force_logout');
}

/**
 * Calls the OS-specific function to force to reboot the machine.
 */
export const forceReboot = async () => {
  await invoke('plugin:power-manager|force_reboot');
}

/**
 * Calls the OS-specific function to force to shut down the machine.
 */
export const forceShutdown = async () => {
  await invoke('plugin:power-manager|force_shutdown');
}

/**
 * Calls the OS-specific function to hibernate the machine.
 */
export const hibernate = async () => {
  await invoke('plugin:power-manager|hibernate');
}

/**
 * Calls the OS-specific function to log out the user.
 */
export const logout = async () => {
  await invoke('plugin:power-manager|logout');
}

/**
 * Calls the OS-specific function to reboot the machine.
 */
export const reboot = async () => {
  await invoke('plugin:power-manager|reboot');
}

/**
 * Calls the OS-specific function to shut down the machine.
 */
export const shutdown = async () => {
  await invoke('plugin:power-manager|shutdown');
}

/**
 * Calls the OS-specific function to put the machine to sleep.
 */
export const sleep = async () => {
  await invoke('plugin:power-manager|sleep');
}