use std::thread::sleep;
use std::time::Duration;
use mki::Keyboard;
use crate::{global, window};

#[tauri::command]
pub async fn insert( text: String) {
    global::clipboard_set(text.as_str());
    global::window_main().hide().unwrap();
    sleep(Duration::from_millis(100));
    Keyboard::LeftControl.press();
    Keyboard::V.press();
    sleep(Duration::from_millis(2));
    Keyboard::V.release();
    Keyboard::LeftControl.release();
}

pub fn translate(text: String) {
    let window = global::window_main();
    window.emit("main://translate", text).unwrap();
    if global::state_get("main-pin").as_bool() == Some(true) { return; }
    window.set_size(window::size(280,168)).unwrap();
    window.set_position(window::pos_by_cursor("main")).unwrap();
    window.show().unwrap();
    window.set_focus().unwrap();
}