use crate::{global_val, global_val_set};
use crate::app::application::Application;
use crate::global::RickApp;
use crate::utils::GlobalVal;


global_val!(GLOBAL_APPLICATION, Application);

/// 初始化应用程序
pub fn init_application(app: RickApp) -> Application {
    global_val_set!(GLOBAL_APPLICATION, Application::new(app));
    // 加载模块
    let app = get_application();
    init_modules(&app);
    app
}

/// 获取应用程序
pub fn get_application() -> Application {
    let val = global_val!(GLOBAL_APPLICATION);
    val.get_ref().unwrap().clone()
}

/// 初始化应用该程序
pub fn init_modules(app: &Application) {

}