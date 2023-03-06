use std::thread::sleep;
use std::time::Duration;
use mki::Keyboard;
use tauri::{AppHandle, ClipboardManager, Manager, Runtime};
use crate::global;

#[tauri::command]
pub async fn insert(label: String, text: String) {
    let app = global::get_app_handle();
    let window = app.get_window(label.as_str()).unwrap();
    let mut clipboard = app.clipboard_manager();
    clipboard.write_text(text).unwrap();
    window.hide().unwrap();
    sleep(Duration::from_millis(100));
    Keyboard::LeftControl.press();
    Keyboard::V.press();
    sleep(Duration::from_millis(2));
    Keyboard::V.release();
    Keyboard::LeftControl.release();
}
