use std::collections::HashMap;
use std::fmt::Formatter;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use mki::Keyboard;
use once_cell::sync::OnceCell;
use tauri::{ClipboardManager, GlobalShortcutManager, Manager};
use crate::{global, window};

pub fn setup() {
    // register_take_words(ModKey::CtrlAlt, Key::D);
}

fn trigger_translate(old_val: String) {
    thread::spawn(move || {
        let app = global::get_app_handle();
        let mut clipboard = app.clipboard_manager();
        thread::sleep(Duration::from_micros(50));
        let mut copy = clipboard.read_text().unwrap_or(Some(String::new())).unwrap_or(String::new());
        let window = app.get_window("main").unwrap();
        if copy == format!("xiaoyi://flags") { copy = format!("") };
        window.emit("main://translate", copy).unwrap();
        clipboard.write_text(old_val).unwrap();
        if global::state_get(format!("main://pin")) == Some(format!("1")) { return; }
        window.set_size(window::initial_size("main")).unwrap();
        window.set_position(window::pos_by_cursor("main")).unwrap();
        window.show().unwrap();
        window.set_focus().unwrap();
    });
}

fn register_take_words(hotkey:String) {
    let sm = global::get_app_handle().global_shortcut_manager();
    // sm.register()
}

fn action_take_words() {
    let last_trigger = global::state_get("main:last-trigger".to_string())
        .unwrap_or("0".to_string()).parse::<u128>().unwrap_or(0u128);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    if now - last_trigger < 1000u128 { return; }
    global::state_set("main:last-trigger".to_string(), now.to_string());

    let mut clipboard = global::get_app_handle().clipboard_manager();
    let old = clipboard.read_text().unwrap_or(Some(String::new())).unwrap_or(String::new());
    clipboard.write_text(format!("xiaoyi://flag")).unwrap();

    Keyboard::D.release();
    Keyboard::LeftAlt.release();
    Keyboard::LeftShift.release();
    // Keyboard::LeftAlt.release();
    // Keyboard::LeftControl.release();

    Keyboard::LeftControl.press();
    Keyboard::C.press();
    thread::sleep(Duration::from_micros(10));
    Keyboard::C.release();
    Keyboard::LeftControl.release();

    // Keyboard::LeftControl.press();
    Keyboard::LeftAlt.press();
    Keyboard::LeftShift.press();

    trigger_translate(old);
}


struct Hotkey {
    modkeys: HashMap<String, String>,
    keys: HashMap<String, String>,
}

static MODKEYS: [(&str, &str); 4] = [
    ("Ctrl+Alt", "Ctrl+Alt"),
    ("Ctrl+Shift", "Ctrl+Shift"),
    ("Shift+Alt", "Shift+Alt"),
    ("Ctrl+Alt+Shift", "Ctrl+Alt+Shift"),
];

static KEYS: [(&str, &str); 95] = [
    ("`", "`"),
    ("[", "["),
    ("]", "]"),
    (",", ","),
    ("0", "0"),
    ("1", "1"),
    ("2", "2"),
    ("3", "3"),
    ("4", "4"),
    ("5", "5"),
    ("6", "6"),
    ("7", "7"),
    ("8", "8"),
    ("9", "9"),
    ("NUM0", "NUM0"),
    ("NUM1", "NUM1"),
    ("NUM2", "NUM2"),
    ("NUM3", "NUM3"),
    ("NUM4", "NUM4"),
    ("NUM5", "NUM5"),
    ("NUM6", "NUM6"),
    ("NUM7", "NUM7"),
    ("NUM8", "NUM8"),
    ("NUM9", "NUM9"),
    ("=", "="),
    ("-", "-"),
    (".", "."),
    ("'", "'"),
    ("\\", "\\"),
    ("A", "A"),
    ("B", "B"),
    ("C", "C"),
    ("D", "D"),
    ("E", "E"),
    ("F", "F"),
    ("G", "G"),
    ("H", "H"),
    ("I", "I"),
    ("J", "J"),
    ("K", "K"),
    ("L", "L"),
    ("M", "M"),
    ("N", "N"),
    ("O", "O"),
    ("P", "P"),
    ("Q", "Q"),
    ("R", "R"),
    ("S", "S"),
    ("T", "T"),
    ("U", "U"),
    ("V", "V"),
    ("W", "W"),
    ("X", "X"),
    ("Y", "Y"),
    ("Z", "Z"),
    (";", ";"),
    ("/", "/"),
    ("Backspace", "BACKSPACE"),
    ("CapsLock", "CAPSLOCK"),
    ("Enter", "ENTER"),
    ("Space", "SPACE"),
    ("Tab", "TAB"),
    ("Delete", "DELETE"),
    ("End", "END"),
    ("Home", "HOME"),
    ("PageDown", "PAGEDOWN"),
    ("PageUp", "PAGEUP"),
    ("↓", "DOWN"),
    ("↑", "UP"),
    ("←", "LEFT"),
    ("→", "RIGHT"),
    ("NumLock", "NUMLOCK"),
    ("NUM+", "NUMADD"),
    ("NUMBackspace", "NUMBACKSPACE"),
    ("NUMDelete", "NUMCLEAR"),
    ("NUM,", "NUMCOMMA"),
    ("NUM/", "NUMDIVIDE"),
    ("NUM-", "NUMSUBSTRACT"),
    ("NUMEnter", "NUMENTER"),
    ("Esd", "ESC"),
    ("PrintScreen", "PRINTSCREEN"),
    ("ScrollLock", "SCROLLLOCK"),
    ("Pause", "PAUSE"),
    ("F1", "F1"),
    ("F2", "F2"),
    ("F3", "F3"),
    ("F4", "F4"),
    ("F5", "F5"),
    ("F6", "F6"),
    ("F7", "F7"),
    ("F8", "F8"),
    ("F9", "F9"),
    ("F10", "F10"),
    ("F11", "F11"),
    ("F12", "F12"),
];