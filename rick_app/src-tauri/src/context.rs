use crate::app::application::Application;
use crate::global::RickApp;
use crate::modules::init_modules;
use crate::service::init_service;
use crate::utils::GlobalVal;
use crate::{global_val, global_val_set};

global_val!(GLOBAL_APPLICATION, Application);

/// 初始化应用程序
pub fn init_application(app: RickApp) -> Application {
    global_val_set!(GLOBAL_APPLICATION, Application::new(app));
    // 加载模块
    let app = get_application();
    let app = init_modules(app);
    let app = init_service(app);
    app
}

/// 获取应用程序
pub fn get_application() -> Application {
    let val = global_val!(GLOBAL_APPLICATION);
    val.get_ref().unwrap().clone()
}
