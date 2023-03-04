#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod key;
mod global;
mod tray;
mod window;

use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, generate_context};
use window_shadows::set_shadow;
use window_vibrancy::{apply_blur};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            global::set_app_handle(app.handle());
            key::setup();
            window::setup();
            Ok(())
        })
        .system_tray(tray::system_tray())
        .on_system_tray_event(|app, event| tray::action(app, event))
        .invoke_handler(tauri::generate_handler![
            global::state_get,
            global::state_set,
            window::start_move
        ])
        .plugin(tauri_plugin_sqlite::init())
        .run(generate_context!())
        .expect("error while running tauri application");
}
