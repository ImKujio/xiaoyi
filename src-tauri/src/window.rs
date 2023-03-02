use mki::{Action, bind_button, InhibitEvent, Mouse, remove_button_bind, State};
use tauri::{PhysicalPosition, PhysicalSize};
use crate::global;

#[tauri::command]
pub fn start_move(label: String) {
    let window = global::get_window(label.clone().as_str()).unwrap();
    window.emit("tauri://move", "start").unwrap();
    bind_button(Mouse::Left, Action {
        callback: Box::new(move |e, s| {
            if let State::Released = s {
                global::get_window(label.as_str()).unwrap().emit("tauri://move", "end").unwrap();
                remove_button_bind(Mouse::Left)
            }
        }),
        inhibit: InhibitEvent::Yes,
        defer: false,
        sequencer: false,
    });
    window.start_dragging().unwrap();
}

use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetCursorPos, GetDC};
use winapi::um::wingdi::{GetDeviceCaps, HORZRES, VERTRES};

pub fn pos_by_cursor(label: &str) -> PhysicalPosition<i32> {
    let mut pos = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut pos) };
    let dis = unsafe {
        let hdc = GetDC(std::ptr::null_mut());
        (GetDeviceCaps(hdc, HORZRES) as i32, GetDeviceCaps(hdc, VERTRES) as i32)
    };
    let isize = initial_size(label);
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

pub fn initial_size(label: &str) -> PhysicalSize<i32> {
    let window = global::get_window(&label).unwrap();
    let scale = window.scale_factor().unwrap();
    let size = ((280f64 * scale).round() as i32, (168f64 * scale).round() as i32);
    PhysicalSize { width: size.0, height: size.1 }
}