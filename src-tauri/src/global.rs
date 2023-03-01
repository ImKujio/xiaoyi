use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;
use tauri::Manager;

type AppHandle = tauri::AppHandle<tauri::Wry>;
type Window = tauri::Window<tauri::Wry>;

static APP_HANDLE: OnceCell<Arc<Mutex<Option<AppHandle>>>> = OnceCell::new();

fn _app_handle() -> &'static Arc<Mutex<Option<AppHandle>>> {
    APP_HANDLE.get_or_init(|| Arc::new(Mutex::new(None)))
}

pub fn set_app_handle(app_handle: AppHandle) {
    *_app_handle().clone().lock().unwrap().deref_mut() = Some(app_handle)
}

pub fn get_app_handle() -> Option<AppHandle> {
    _app_handle().clone().lock().unwrap().deref().as_ref()
        .map(|a| a.to_owned())
}

pub fn get_window(label: &str) -> Option<Window> {
    _app_handle().clone().lock().unwrap().deref().as_ref()
        .map(|a| a.get_window(label).unwrap())
}


type State = HashMap<String, String>;

static STATE: OnceCell<Arc<Mutex<State>>> = OnceCell::new();

fn _state() -> &'static Arc<Mutex<State>> {
    STATE.get_or_init(|| Arc::new(Mutex::new(State::new())))
}

#[tauri::command]
pub fn state_set(key: String, val: String) {
    _state().clone().lock().unwrap().deref_mut().insert(key, val);
}

#[tauri::command]
pub fn state_get(key: String) -> Option<String> {
    _state().clone().lock().unwrap().deref().get(&key).cloned()
}