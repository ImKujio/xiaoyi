use mki::{Action, bind_button, InhibitEvent, Mouse, remove_button_bind, State};
use tauri::{Manager, PhysicalPosition, PhysicalSize, UserAttentionType, WindowEvent};
use crate::{global};


pub fn setup() {
    let window_main = global::window_main();
    if let Err(_) = set_shadow(&window_main, true) {}
    if let Err(_) = apply_blur(&window_main, Some((255, 255, 255, 200))) {}
    window_main.set_size(main_size()).unwrap();
    window_main.on_window_event(move |e| {
        if let WindowEvent::Focused(focus) = e {
            if focus.to_owned() { return; }
            if global::state_get("main-pin").as_bool() == Some(true) { return; }
            if global::state_get("main-moving").as_bool() == Some(true) { return; }
            window_main.hide().unwrap();
        }
    });
}

pub fn settings_window() {
    let app = global::app();
    if let Some(window_settings) = app.get_window("settings") {
        window_settings.show().unwrap();
        window_settings.set_focus().unwrap();
        window_settings.request_user_attention(Some(UserAttentionType::Informational)).unwrap()
    }
    if let None = app.get_window("settings") {
        let window_settings = tauri::WindowBuilder::new(
            app,
            "settings",
            tauri::WindowUrl::App("settings.html".into()),
        ).title(format!("小译设置"))
            .resizable(false)
            .build().unwrap();
        window_settings.set_size(size(800, 600)).unwrap();
        window_settings.set_max_size(Some(size(800, 600))).unwrap();
        window_settings.set_min_size(Some(size(800, 600))).unwrap();
        window_settings.on_window_event(|e| {
            if let WindowEvent::CloseRequested { .. } = e {
                global::settings_store();
            }
        });
    }
}

#[tauri::command]
pub fn start_move(label: String) {
    let window = global::app().get_window(label.as_str()).unwrap();
    global::state_set(format!("{}-moving", label).as_str(), JsonValue::Bool(true));
    bind_button(Mouse::Left, Action {
        callback: Box::new(move |e, s| {
            if let State::Released = s {
                global::state_set(format!("{}-moving", label).as_str(), JsonValue::Bool(false));
                remove_button_bind(Mouse::Left)
            }
        }),
        inhibit: InhibitEvent::No,
        defer: false,
        sequencer: false,
    });
    window.start_dragging().unwrap();
}

use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetCursorPos, GetDC};
use winapi::um::wingdi::{GetDeviceCaps, HORZRES, VERTRES};
use window_shadows::set_shadow;
use window_vibrancy::apply_blur;
use crate::global::JsonValue;

pub fn pos_by_cursor(label: &str) -> PhysicalPosition<i32> {
    let mut pos = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut pos) };
    let dis = unsafe {
        let hdc = GetDC(std::ptr::null_mut());
        (GetDeviceCaps(hdc, HORZRES) as i32, GetDeviceCaps(hdc, VERTRES) as i32)
    };
    let isize = main_size();
    if pos.x + isize.width > dis.0 && pos.y + isize.height > dis.1 {
        PhysicalPosition { x: pos.x - isize.width, y: pos.y - isize.height }
    } else if pos.x + isize.width > dis.0 {
        PhysicalPosition { x: pos.x - isize.width, y: pos.y }
    } else if pos.y + isize.height > dis.1 {
        PhysicalPosition { x: pos.x, y: pos.y - isize.height }
    } else {
        PhysicalPosition { x: pos.x, y: pos.y }
    }
}

pub fn size(w: i32, h: i32) -> PhysicalSize<i32> {
    let scale = global::window_main().scale_factor().unwrap();
    let size = ((w as f64 * scale).round() as i32, (h as f64 * scale).round() as i32);
    PhysicalSize { width: size.0, height: size.1 }
}

pub fn main_size() -> PhysicalSize<i32> {
    let w = global::settings_get("width").as_i64().unwrap_or(280) as i32;
    let h = global::settings_get("height").as_i64().unwrap_or(168) as i32;
    size(w, h)
}