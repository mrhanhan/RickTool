mod app_group;

use crate::app::service::ServiceRegister;

/// 服务注册
pub fn init_service(_register: &ServiceRegister) {
    // 注册分组服务
    app_group::init_service(_register);
}