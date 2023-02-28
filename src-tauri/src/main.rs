#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod hotkey;
mod global;

use std::process::exit;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, generate_context, LogicalPosition, PhysicalPosition, Assets, AppHandle, Runtime, Wry};
use window_shadows::set_shadow;
use window_vibrancy::{apply_blur, NSVisualEffectMaterial};
use crate::global::Handle;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let settings = CustomMenuItem::new("setting".to_string(), "设置");
    let tray_menu = SystemTrayMenu::new().add_item(settings).add_item(quit);
    tauri::Builder::default()
        .setup(|app| {
            Handle::get().set_app_handle(app.handle());
            let window = app.get_window("main").expect("get window error");
            set_shadow(&window, true).expect("error while set_shadow");
            apply_blur(&window, Some((255, 255, 255, 200))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            hotkey::setup();
            Ok(())
        })
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::LeftClick { .. } = event {
                let window = app.get_window("main").unwrap();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.center().unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            };
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                if let "quit" = id.as_str() {
                    exit(0)
                };
                if let "setting" = id.as_str() {}
            };
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(generate_context!())
        .expect("error while running tauri application");
}
