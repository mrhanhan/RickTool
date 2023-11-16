// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod context;
mod global;
pub mod model;
mod modules;
mod seq;
mod service;
mod store;
mod utils;

use crate::app::application::{Application, ApplicationEvent};
use crate::context::{get_application, init_application};
use crate::global::RickInvoke;

use log::{Level, LevelFilter, Metadata, Record};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let line = record.line().unwrap_or(0);
            println!("[{}] {}\n{}", record.level(), record.module_path().unwrap_or(" - "), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&SimpleLogger {}).map(|l| {
        log::set_max_level(LevelFilter::Debug);
    });
}

fn handler(invoke: RickInvoke) {
    get_application().service_register().async_call(invoke);
}

fn main() {
    init();
    let app = tauri::Builder::default()
        .invoke_handler(handler)
        .build(tauri::generate_context!())
        .unwrap();
    let mut app = init_application(app);
    app.event_context()
        .on_into(ApplicationEvent::Started, |_data: &Application| {
            println!("启动啦: App")
        });
    app.start();
}
