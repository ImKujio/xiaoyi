use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, Wry};
use crate::{global, window};
use crate::global::JsonValue;
use crate::utils::now_timestamp;

pub fn system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let settings = CustomMenuItem::new("setting".to_string(), "设置");
    let tray_menu = SystemTrayMenu::new().add_item(settings).add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

pub fn action(app: &AppHandle<Wry>, event: SystemTrayEvent) {
    if let SystemTrayEvent::LeftClick { .. } = event {
        let last = global::state_get("tray-debounce").as_u64().unwrap_or(0u64);
        let now = now_timestamp();
        if now - last < 1000u64 { return; }
        let window = app.get_window("main").unwrap();
        if !window.is_visible().unwrap() {
            window.center().unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        global::state_set("tray-debounce",JsonValue::from(now))
    };
    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        if let "quit" = id.as_str() {
            app.exit(0);
        };
        if let "setting" = id.as_str() {
            window::settings_window()
        }
    };
}