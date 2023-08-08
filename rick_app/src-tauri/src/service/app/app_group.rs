use rick_core::sqlite::{QueryDatabaseOperate, SqlWrapper};
use crate::app::service::ServiceRegister;
use crate::global::{RickResult};
use crate::store::app::AppGroup;


/// 服务注册
pub fn init_service(_register: &ServiceRegister) {
    _register.register_closure_fn("save_app_group", save_app_group);
    _register.register_run_fn("list_app_group", list_app_group);
}
/// 保存分组
fn save_app_group(app: AppGroup) -> RickResult<AppGroup> {
    Ok(app)
}

fn list_app_group() -> RickResult<Vec<AppGroup>> {
    match AppGroup::select_list(&SqlWrapper::new()) {
        Ok(_data) => Ok(_data),
        Err(ref _err) => Err(_err.into())
    }
}