use crate::app::service::ServiceRegister;

mod files;

/// 服务注册
pub fn init_service(_register: &ServiceRegister) {
    files::init_service(_register);
}