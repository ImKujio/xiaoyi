use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;
use tauri::{AppHandle, Manager, Window, Wry};

pub struct Handle {
    app_handle: Arc<Mutex<Option<AppHandle<Wry>>>>,
}

static HANDLE: OnceCell<Handle> = OnceCell::new();

impl Handle {
    pub fn get() -> &'static Handle {
        HANDLE.get_or_init(|| Handle {
            app_handle: Arc::new(Mutex::new(None))
        })
    }

    pub fn set_app_handle(&self, app_handle: AppHandle<Wry>) {
        *self.app_handle.clone().lock().unwrap() = Some(app_handle)
    }

    pub fn get_app_handle(&self) -> Option<AppHandle<Wry>> {
        self.app_handle.lock().unwrap().clone()
    }

    pub fn get_window(&self, label: &str) -> Option<Window<Wry>> {
        self.get_app_handle().map_or(None, |a| Some(a.get_window(label).unwrap()))
    }
}