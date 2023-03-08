#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

extern crate core;

use std::{fs, panic};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

use chrono::Local;
use tauri::{App, generate_context, Wry};

mod hotkey;
mod global;
mod tray;
mod window;
mod utils;
mod action;

fn handle_panic(app: &App<Wry>){
    let mut path = app.path_resolver().app_data_dir().unwrap();
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }
    path.push("error.log");
    let app = app.handle();
    panic::set_hook(Box::new(move |info|{
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path.clone()).unwrap();
        let now = Local::now().format("%Y-%m-%d %H:%M:%S:%3f").to_string();
        let mut buf = BufWriter::new(file);
        let info = format!("[{}]:{}\n",now,info);
        buf.write_all( info.as_bytes()).unwrap();
        buf.flush().unwrap();
        app.exit(555)
    }));
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            println!("{:?}", app.path_resolver().resource_dir());
            handle_panic(app);
            global::setup(app.handle());
            hotkey::setup();
            window::setup();
            Ok(())
        })
        .system_tray(tray::system_tray())
        .on_system_tray_event(|app, event| tray::action(app, event))
        .invoke_handler(tauri::generate_handler![
            global::state_post,
            global::state_fetch,
            window::start_move,
            action::insert
        ])
        .plugin(tauri_plugin_sqlite::init())
        .run(generate_context!())
        .expect("error while running tauri application");
}
