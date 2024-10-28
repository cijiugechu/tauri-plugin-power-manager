const COMMANDS: &[&str] = &[
    "force_logout",
    "force_reboot",
    "force_shutdown",
    "hibernate",
    "logout",
    "reboot",
    "shutdown",
    "sleep",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
