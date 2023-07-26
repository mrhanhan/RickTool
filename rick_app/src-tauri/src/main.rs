// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod context;
mod utils;
mod app;
mod global;

use std::any::Any;
use crate::app::application::{Application, ApplicationEvent};
use crate::context::{get_application, init_application};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!()).unwrap();
    init_application(app);
    let event_context = get_application().event_context();
    event_context.on_into(ApplicationEvent::Started, |_data: &i32|{
        println!("启动啦")
    });
    event_context.on_into(ApplicationEvent::Started, |_data: &Application|{
        println!("启动啦: App")
    });

    get_application().start();
}
