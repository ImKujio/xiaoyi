use std::thread;
use std::time::Duration;
use clipboard_win::{formats, get_clipboard};
use mki::{Action, bind_key, InhibitEvent, Keyboard, State};
use tauri::{ PhysicalPosition, PhysicalSize, Position, Runtime, Size, Window};
use winapi::um::winuser::{GetCursorPos, GetDC};
use winapi::um::wingdi::{GetDeviceCaps, HORZRES, VERTRES};
use winapi::shared::windef::POINT;
use crate::global;

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

pub fn setup() {
    bind_key(
        Keyboard::D,
        Action {
            callback: Box::new(|e, s| {
                // 如果T键按下
                if let State::Pressed = s {
                    // 如果Ctrl和Alt也按下
                    if Keyboard::LeftControl.is_pressed() && Keyboard::LeftAlt.is_pressed() {
                        // 释放T键和Alt键
                        Keyboard::LeftAlt.release();
                        // 按下C键并释放
                        Keyboard::C.press();
                        Keyboard::C.release();
                        Keyboard::LeftAlt.press();
                        // 等待系统处理复制操作，
                        thread::sleep(Duration::from_millis(10));
                        // 读取剪贴板
                        let copy: String = get_clipboard(formats::Unicode).unwrap_or(String::new());
                        // 获取窗口
                        if let Some(window) = global::get_window("main") {
                            // 获取窗口信息
                            let window_info = WindowInfo::new(&window);
                            window.set_size(window_info.size).unwrap();
                            // 获取鼠标位置并更新窗口位置
                            window.set_position(window_info.pos).unwrap();
                            // 发送剪贴板内容
                            window.emit("translate", copy).unwrap();
                            // 显示窗口并聚焦
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                }
            }),
            inhibit: InhibitEvent::maybe(|| {
                if Keyboard::LeftControl.is_pressed() && Keyboard::LeftAlt.is_pressed() {
                    InhibitEvent::Yes
                } else {
                    InhibitEvent::No
                }
            }),
            defer: false,
            sequencer: false,
        },
    );
}