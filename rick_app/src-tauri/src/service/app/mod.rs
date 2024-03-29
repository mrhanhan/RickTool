mod app;
mod app_group;
mod app_runtime;
mod app_start;

use crate::app::service::ServiceRegister;
pub use app::get_logo_dir;
/// 服务注册
pub fn init_service(_register: &ServiceRegister) {
    // 注册分组服务
    app_group::init_service(_register);
    app_runtime::init_service(_register);
    app::init_service(_register);
    app_start::init_service(_register);
}
