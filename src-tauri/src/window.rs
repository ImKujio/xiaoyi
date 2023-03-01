use mki::{Action, bind_button, InhibitEvent, Mouse, remove_button_bind, State};
use tauri::{AppHandle, PhysicalPosition, PhysicalSize, Runtime};
use crate::global;

pub struct LabelWindow {
    label: String,
    window: tauri::Window<tauri::Wry>,
}

pub fn form_label(label: &str) -> LabelWindow {
    LabelWindow {
        label: label.to_string(),
        window: global::get_window(label).unwrap(),
    }
}

impl LabelWindow {
    fn pos(&self) -> PhysicalPosition<i32> {
        self.window.outer_position().unwrap()
    }
    fn size(&self) -> PhysicalSize<u32> {
        self.window.outer_size().unwrap()
    }
    fn scale(&self) -> f64 {
        self.window.scale_factor().unwrap_or(1f64)
    }
}

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

#[tauri::command]
pub fn window_resize(label:String,width:i32,height:i32){
    let window = form_label(&label);
    let size = window.size();
    let scale = window.scale();

}
