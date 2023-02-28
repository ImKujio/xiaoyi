use std::thread;
use std::time::Duration;
use clipboard_win::{formats, get_clipboard};
use mki::{Action, bind_key, InhibitEvent, Keyboard, State};
use tauri::{App, Manager, PhysicalSize, Runtime, Size, Window};
use crate::utils::window_pos;


pub fn setup<R: Runtime>(app: &mut App<R>) {
    let app = Box::leak(Box::new(app.app_handle()));
    bind_key(
        Keyboard::D,
        Action {
            callback: Box::new(|e, s| {
                // 如果T键按下
                if let State::Pressed = s {
                    println!("d is pressed");
                    // 如果Ctrl和Alt也按下
                    if Keyboard::LeftControl.is_pressed() && Keyboard::LeftAlt.is_pressed() {
                        // 释放T键和Alt键
                        // Keyboard::D.release();
                        Keyboard::LeftAlt.release();
                        // 按下C键并释放
                        Keyboard::C.press();
                        Keyboard::C.release();
                        Keyboard::LeftAlt.press();
                        // Keyboard::LeftControl.release();
                        // Keyboard::LeftControl.release();
                        // 等待系统处理复制操作，
                        thread::sleep(Duration::from_millis(10));
                        // 读取剪贴板
                        let copy: String = get_clipboard(formats::Unicode).unwrap_or(String::new());
                        // 获取窗口
                        let window = &app.get_window("main").unwrap();
                        // 获取窗口位置
                        window.set_size(Size::Physical(PhysicalSize::from((280, 168)))).unwrap();
                        let size = window.outer_size().unwrap();
                        let size = (size.width as i32, size.height as i32);
                        // 获取鼠标位置并更新窗口位置
                        window.set_position(window_pos(size, false)).unwrap();
                        // 发送剪贴板内容
                        window.emit("translate", copy).unwrap();
                        // 显示窗口并聚焦
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
            }),
            inhibit: InhibitEvent::maybe(|| {
                println!("left ctrl is:{}", Keyboard::LeftControl.is_pressed());
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