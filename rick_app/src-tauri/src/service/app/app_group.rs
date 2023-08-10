use rick_core::sqlite::{QueryDatabaseOperate, SqlWrapper, UpdateDatabaseOperate};
use crate::app::service::ServiceRegister;
use crate::global::{RickResult};
use crate::seq::{increase_table};
use crate::service::{common_result, convert_result};
use crate::store::app::AppGroup;


/// 服务注册
pub fn init_service(_register: &ServiceRegister) {
    _register.register_closure_fn("save_app_group", save_app_group);
    _register.register_run_fn("list_app_group", list_app_group);
}
/// 保存分组
fn save_app_group(mut app: AppGroup) -> RickResult<AppGroup> {
    app.id = increase_table(&app);
    common_result(AppGroup::save(&app), app)
}

fn list_app_group() -> RickResult<Vec<AppGroup>> {
    convert_result(AppGroup::select_list(&SqlWrapper::new()))
}