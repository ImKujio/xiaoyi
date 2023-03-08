use std::{fs, thread};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use once_cell::sync::OnceCell;
use tauri::{AppHandle, ClipboardManager, Manager, Window, Wry};

pub type JsonValue = serde_json::Value;

pub fn setup(app: AppHandle<Wry>) {
    WINDOW_MAIN.set(app.get_window("main").unwrap()).unwrap();
    STATE.set(Arc::new(Mutex::new(HashMap::new()))).unwrap();
    APP.set(app).unwrap();
    settings_init()
}

static APP: OnceCell<AppHandle<Wry>> = OnceCell::new();

pub fn app() -> &'static AppHandle<Wry> {
    APP.get().unwrap()
}

static WINDOW_MAIN: OnceCell<Window<Wry>> = OnceCell::new();

pub fn window_main() -> &'static Window<Wry> {
    WINDOW_MAIN.get().unwrap()
}

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

static SETTINGS: OnceCell<Arc<Mutex<HashMap<String, JsonValue>>>> = OnceCell::new();

fn settings_cus_path() -> PathBuf {
    let mut path = app().path_resolver().app_config_dir().unwrap();
    if !path.exists() { fs::create_dir_all(&path).unwrap(); }
    path.push("settings.json");
    path
}

fn settings_def_path() -> PathBuf {
    let mut path = app().path_resolver().resource_dir().unwrap();
    path.push("settings.json");
    path
}

fn settings_init() {
    let settings_def = fs::read_to_string(settings_def_path()).unwrap_or("".to_string());
    let mut settings: HashMap<String, JsonValue> = serde_json::from_str(&settings_def).unwrap_or(HashMap::new());
    let settings_cus = fs::read_to_string(settings_cus_path()).unwrap_or("".to_string());
    let settings_cus: HashMap<String, JsonValue> = serde_json::from_str(&settings_cus).unwrap_or(HashMap::new());
    settings.extend(settings_cus);
    SETTINGS.set(Arc::new(Mutex::new(settings))).unwrap();
}

pub fn settings_store() {
    let mut settings = app().path_resolver().app_config_dir().unwrap();
    if !settings.exists() { fs::create_dir_all(&settings).unwrap(); }
    settings.push("settings.json");
    let settings = OpenOptions::new().create(true).write(true).append(false).open(settings).unwrap();
    let mut settings = BufWriter::new(settings);
    let settings_map = SETTINGS.get().unwrap().clone();
    let settings_txt = serde_json::to_string_pretty(settings_map.lock().unwrap().deref()).unwrap();
    settings.write_all(settings_txt.as_bytes()).unwrap();
    settings.flush().unwrap();
}

pub async fn settings_post(key: &str, val: JsonValue) {
    SETTINGS.get().unwrap().clone().lock().unwrap().insert(key.to_string(), val);
}

pub async fn settings_fetch(key: &str) -> JsonValue {
    match SETTINGS.get().unwrap().clone().lock().unwrap().get(key) {
        None => { JsonValue::Null }
        Some(v) => { v.clone() }
    }
}

pub fn settings_set(key: &str, val: JsonValue) {
    SETTINGS.get().unwrap().clone().lock().unwrap().insert(key.to_string(), val);
}

pub fn settings_get(key: &str) -> JsonValue {
    match SETTINGS.get().unwrap().clone().lock().unwrap().get(key) {
        None => { JsonValue::Null }
        Some(v) => { v.clone() }
    }
}
