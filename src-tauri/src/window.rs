use mki::{Action, bind_button, InhibitEvent, Mouse, remove_button_bind, State};
use tauri::{PhysicalPosition, PhysicalSize, WindowEvent};
use crate::{global};


pub fn setup() {
    let window_main = global::get_window("main");
    if let Err(_) = set_shadow(&window_main, true) {}
    if let Err(_) = apply_blur(&window_main, Some((255, 255, 255, 200))) {}
    window_main.on_window_event(|e| {
        if let WindowEvent::Focused(focus) = e {
            if focus.to_owned() { return; }
            if global::state_get(format!("main://pin")) == Some(format!("1")){ return;}
            if global::state_get(format!("main://moving")) == Some(format!("1")) {return;}
            global::get_window("main").hide().unwrap();
        }
    });
    // disable_window_animations("main");
}

#[tauri::command]
pub fn start_move(label: String) {
    let window = global::get_window(label.clone().as_str());
    global::state_set(format!("{}://moving", label), "1".to_string());
    bind_button(Mouse::Left, Action {
        callback: Box::new(move |e, s| {
            if let State::Released = s {
                global::state_set(format!("{}://moving", label), "0".to_string());
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
use window_shadows::set_shadow;
use window_vibrancy::apply_blur;

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
    let window = global::get_window(&label);
    let scale = window.scale_factor().unwrap();
    let size = ((280f64 * scale).round() as i32, (168f64 * scale).round() as i32);
    PhysicalSize { width: size.0, height: size.1 }
}

// use winapi::um::winuser::GetAncestor;
// use winapi::um::winuser::GetWindowLongPtrW;
// use winapi::um::winuser::GWL_STYLE;
// use winapi::um::winuser::SetWindowLongPtrW;
// use winapi::um::winuser::SWP_DRAWFRAME;
// use winapi::um::winuser::SWP_FRAMECHANGED;
// use winapi::um::winuser::SWP_NOACTIVATE;
// use winapi::um::winuser::SWP_NOMOVE;
// use winapi::um::winuser::SWP_NOREPOSITION;
// use winapi::um::winuser::SWP_NOZORDER;
// use winapi::um::winuser::HWND_TOPMOST;

// fn disable_window_animations(label: &str) {
//     let window = global::get_window(label).unwrap();
//     let hwnd = window.A().unwrap();
//     let style = unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) };
//     let new_style = style & !(0x0002 | 0x0001); // remove WS_BORDER and WS_CAPTION
//     unsafe {
//         SetWindowLongPtrW(hwnd, GWL_STYLE, new_style);
//
//         let mut rect = std::mem::zeroed();
//         let result = GetWindowRect(hwnd, &mut rect);
//         if result == 0 {
//             return; // failed to get the window position and size
//         }
//         let width = rect.right - rect.left;
//         let height = rect.bottom - rect.top;
//         let x = rect.left;
//         let y = rect.top;
//         let flags = SWP_NOMOVE | SWP_NOZORDER | SWP_NOREPOSITION | SWP_NOACTIVATE | SWP_FRAMECHANGED | SWP_DRAWFRAME;
//         SetWindowPos(hwnd, HWND_TOPMOST, x, y, width, height, flags);
//     }
// }