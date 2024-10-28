# Tauri Plugin power-manager

This plugin provides a cross platform way to shut down, reboot or log out operations.

Supported platforms: `Linux`, `Windows` and `macOS`.

## Installation

To install the Rust plugin, run:

```bash
cargo add tauri-plugin-power-manager
```

Then, load the plugin in your Tauri app as follows:

```rust
tauri::Builder::default()
        .plugin(tauri_plugin_power_manager::init()) // THIS LINE
        // More builder methods
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
```

Finally, install the JS client bindings:

```bash
pnpm add tauri-plugin-power-manager-api
``` 

## API

```typescript
/**
 * Calls the OS-specific function to force to log out the user.
 */
export declare const forceLogout: () => Promise<void>;
/**
 * Calls the OS-specific function to force to reboot the machine.
 */
export declare const forceReboot: () => Promise<void>;
/**
 * Calls the OS-specific function to force to shut down the machine.
 */
export declare const forceShutdown: () => Promise<void>;
/**
 * Calls the OS-specific function to hibernate the machine.
 */
export declare const hibernate: () => Promise<void>;
/**
 * Calls the OS-specific function to log out the user.
 */
export declare const logout: () => Promise<void>;
/**
 * Calls the OS-specific function to reboot the machine.
 */
export declare const reboot: () => Promise<void>;
/**
 * Calls the OS-specific function to shut down the machine.
 */
export declare const shutdown: () => Promise<void>;
/**
 * Calls the OS-specific function to put the machine to sleep.
 */
export declare const sleep: () => Promise<void>;
```