// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod context;
mod utils;
mod app;
mod global;
mod modules;
mod service;
pub mod model;

use crate::app::application::{Application, ApplicationEvent};
use crate::context::{get_application, init_application};
use crate::global::RickInvoke;

fn handler(invoke: RickInvoke) {
    get_application().service_register().call(invoke);
}

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(handler)
        .build(tauri::generate_context!()).unwrap();
    let mut app = init_application(app);
    app.event_context().on_into(ApplicationEvent::Started, |_data: &Application|{
        println!("启动啦: App")
    });
    app.start();
}
