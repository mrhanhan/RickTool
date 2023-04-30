use tauri::{Builder, generate_context, Wry};

use crate::{context::{use_application, events::Publisher, STOP, START_AFTER, Application}, command};


pub type TauriBuilder = Builder<Wry>;

fn create_app() -> tauri::App {
    let context = generate_context!();
    let mut builder = Builder::default();
    builder = command::register_handler(builder.any_thread());
    builder.on_window_event(|e| {
        match  e.event(){
            tauri::WindowEvent::Destroyed => {
                println!("关闭");
                use_application().stop();
            },
            _ => {
                println!("GlobalWindowEvent: {:?}", e.event());
            }
        };
    }).build(context).unwrap()
}
// 初始化tauri 配置
pub(crate) fn init_tauri_config() { 
    {use_application().add_subscriber(START_AFTER, Box::new(|_, _| {
        println!("准备启动Tauri");
        let app = create_app();
        app.run(|_, _|{});
        println!("准备启动Tauri 结束");
    }));}
    {use_application().add_subscriber(STOP, Box::new(|_, _| {
        println!("Application 停止");
    }));}


}