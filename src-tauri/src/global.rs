use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;
use tauri::{AppHandle, ClipboardManager, Manager, Window, Wry};

pub fn setup(app: AppHandle<Wry>) {
    WINDOW_MAIN.set(app.get_window("main").unwrap()).unwrap();
    APP.set(app).unwrap();
    STATE.set(Arc::new(Mutex::new(HashMap::new()))).unwrap();
}

static APP: OnceCell<AppHandle<Wry>> = OnceCell::new();

pub fn app() -> &'static AppHandle<Wry> {
    APP.get().unwrap()
}

static WINDOW_MAIN: OnceCell<Window<Wry>> = OnceCell::new();

pub fn window_main() -> &'static Window<Wry> {
    WINDOW_MAIN.get().unwrap()
}

pub type JsonValue = serde_json::Value;

static STATE: OnceCell<Arc<Mutex<HashMap<String, JsonValue>>>> = OnceCell::new();

#[tauri::command]
pub async fn state_post(key: String, val: JsonValue) {
    STATE.get().unwrap().clone().lock().unwrap().insert(key, val);
}

#[tauri::command]
pub async fn state_fetch(key: String) -> JsonValue {
    match STATE.get().unwrap().clone().lock().unwrap().get(&key) {
        None => { JsonValue::Null }
        Some(v) => { v.clone() }
    }
}

pub fn state_set(key: &str, val: JsonValue) {
    STATE.get().unwrap().clone().lock().unwrap().insert(key.to_string(), val);
}

pub fn state_get(key: &str) -> JsonValue {
    match STATE.get().unwrap().clone().lock().unwrap().get(key) {
        None => { JsonValue::Null }
        Some(v) => { v.clone() }
    }
}

pub fn clipboard_set(val: &str) {
    app().clipboard_manager().write_text(val.to_string()).unwrap();
}

pub fn clipboard_get() -> String {
    app().clipboard_manager().read_text()
        .unwrap_or(Some("".to_string()))
        .unwrap_or("".to_string())
}