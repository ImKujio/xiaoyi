use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use clipboard_win::{formats, get_clipboard};
use mki::{Action, bind_key, InhibitEvent, Keyboard, register_hotkey, State};
use tauri::{ClipboardManager, GlobalShortcutManager, Manager, PhysicalPosition, PhysicalSize, Position, Runtime, Size, Window};
use winapi::um::winuser::{GetCursorPos, GetDC};
use winapi::um::wingdi::{GetDeviceCaps, HORZRES, VERTRES};
use winapi::shared::windef::POINT;
use crate::{global, window};

struct WindowInfo {
    size: Size,
    pos: Position,
}

impl WindowInfo {
    pub fn new<R: Runtime>(window: &Window<R>) -> WindowInfo {
        let mut pos = POINT { x: 0, y: 0 };
        unsafe { GetCursorPos(&mut pos) };
        let mut dis = (0, 0);
        unsafe {
            let hdc = GetDC(std::ptr::null_mut());
            let w = GetDeviceCaps(hdc, HORZRES);
            let h = GetDeviceCaps(hdc, VERTRES);
            dis = (w, h);
        }
        let scale = window.scale_factor().unwrap();
        let size = ((280f64 * scale).round() as i32, (168f64 * scale).round() as i32);
        let pos = if pos.x + size.0 > dis.0 && pos.y + size.1 > dis.1 {
            Position::Physical(PhysicalPosition { x: pos.x - size.0, y: pos.y - size.1 })
        } else if pos.x + size.0 > dis.0 {
            Position::Physical(PhysicalPosition { x: pos.x - size.0, y: pos.y })
        } else if pos.y + size.1 > dis.1 {
            Position::Physical(PhysicalPosition { x: pos.x, y: pos.y - size.1 })
        } else {
            Position::Physical(PhysicalPosition { x: pos.x, y: pos.y })
        };
        return WindowInfo {
            size: Size::Physical(PhysicalSize { width: size.0 as u32, height: size.1 as u32 }),
            pos,
        };
    }
}

enum ModKey {
    CtrlAlt,
    CtrlShift,
    ShiftAlt,
    CtrlAltShit,
}

fn trigger_translate(old: String) {
    thread::spawn(move || {
        let app = global::get_app_handle().unwrap();
        let mut clipboard = app.clipboard_manager();
        let window = app.get_window("main").unwrap();
        let window_info = WindowInfo::new(&window);
        window.set_size(window_info.size).unwrap();
        window.set_position(window_info.pos).unwrap();
        thread::sleep(Duration::from_micros(10));
        let copy = clipboard.read_text().unwrap_or(Some(String::new())).unwrap_or(String::new());
        window.emit("translate", copy).unwrap();
        window.show().unwrap();
        window.set_focus().unwrap();
        clipboard.write_text(old).unwrap();
    });
}

pub fn setup() {
    let app = global::get_app_handle().unwrap();
    let mut sm = app.global_shortcut_manager();
    sm.register("Ctrl+Alt+D", move || {
        let last_trigger = global::state_get("last-trigger-copy-translate".to_string())
            .unwrap_or("0".to_string()).parse::<u128>().unwrap_or(0u128);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        if now - last_trigger < 1000u128 { return; }
        global::state_set("last-trigger-copy-translate".to_string(),now.to_string());

        let mut clipboard = app.clipboard_manager();
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