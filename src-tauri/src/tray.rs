use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, Wry};

pub fn system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let settings = CustomMenuItem::new("setting".to_string(), "设置");
    let tray_menu = SystemTrayMenu::new().add_item(settings).add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

pub fn action(app: &AppHandle<Wry>, event: SystemTrayEvent) {
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
            app.exit(0);
        };
        if let "setting" = id.as_str() {}
    };
}