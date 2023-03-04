use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use mki::Keyboard;
use tauri::{ClipboardManager, GlobalShortcutManager, Manager};
use crate::{global, window};

// enum ModKey {
//     CtrlAlt,
//     CtrlShift,
//     ShiftAlt,
//     CtrlAltShit,
// }

fn trigger_translate(old_val: String) {
    thread::spawn(move || {
        let app = global::get_app_handle().unwrap();
        let mut clipboard = app.clipboard_manager();
        thread::sleep(Duration::from_micros(20));
        let copy = clipboard.read_text().unwrap_or(Some(String::new())).unwrap_or(String::new());
        let window = app.get_window("main").unwrap();
        window.emit("main://translate", copy).unwrap();
        clipboard.write_text(old_val).unwrap();
        if global::state_get(format!("main://pin")) == Some(format!("1")) { return; }
        window.set_size(window::initial_size("main")).unwrap();
        window.set_position(window::pos_by_cursor("main")).unwrap();
        window.show().unwrap();
        window.set_focus().unwrap();
    });
}

pub fn setup() {
    let app = global::get_app_handle().unwrap();
    let mut sm = app.global_shortcut_manager();
    sm.register("Ctrl+Alt+D", move || {
        let last_trigger = global::state_get("main:last-trigger".to_string())
            .unwrap_or("0".to_string()).parse::<u128>().unwrap_or(0u128);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        if now - last_trigger < 1000u128 { return; }
        global::state_set("main:last-trigger".to_string(), now.to_string());

        let clipboard = app.clipboard_manager();
        let old = clipboard.read_text().unwrap_or(Some(String::new())).unwrap_or(String::new());

        Keyboard::D.release();
        Keyboard::LeftAlt.release();
        Keyboard::LeftControl.release();

        Keyboard::LeftControl.press();
        Keyboard::C.press();
        thread::sleep(Duration::from_micros(2));
        Keyboard::C.release();
        Keyboard::LeftControl.release();

        Keyboard::LeftControl.press();
        Keyboard::LeftAlt.press();

        trigger_translate(old);
    }).unwrap();
}