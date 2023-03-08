use std::thread;
use std::thread::sleep;
use std::time::Duration;

use mki::Keyboard;
use tauri::GlobalShortcutManager;

use crate::{action, global};
use crate::global::JsonValue;
use crate::utils::now_timestamp;

pub fn setup() {
    register_take_words("Ctrl+Alt+D".to_string());
}

fn register<F: Fn() + Send + 'static>(accelerator: &str, handler: F){
    global::app().global_shortcut_manager().register(accelerator, handler).unwrap();
}

fn register_take_words(hotkey:String) {
    register(hotkey.as_str(), move ||{
        let last = global::state_get("take-words-debounce").as_u64().unwrap_or(0u64);
        let now = now_timestamp();
        if now - last < 1000u64 { return; }
        global::state_set("take-words-debounce", JsonValue::from(now));

        let original = global::clipboard_get();
        global::clipboard_set("xiaoyi://flag");

        Keyboard::D.release();
        Keyboard::LeftAlt.release();
        Keyboard::LeftControl.release();

        Keyboard::LeftControl.press();
        Keyboard::C.press();
        sleep(Duration::from_millis(2));
        Keyboard::C.release();
        Keyboard::LeftControl.release();

        Keyboard::LeftControl.press();
        Keyboard::LeftAlt.press();

        thread::spawn(move || {
            sleep(Duration::from_millis(50));
            let copy = global::clipboard_get();
            let copy = if copy == "xiaoyi://flag".to_string() { "".to_string() } else { copy };
            action::translate(copy);
            global::clipboard_set(original.as_str());
        });
    });
}

static MODS: [(&str, &str); 4] = [
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