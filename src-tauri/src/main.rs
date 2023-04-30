#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;

use context::{Application, use_application};

mod store;
mod utils;
mod context;
mod config;
mod model;
mod tests;
mod api;
mod command;

/// load
fn load() {
    // 初始化配置
    config::init_config();
    // 注册API
    api::register_api();
}

fn main() {
    load();
    use_application().init();
    use_application().start();
}
