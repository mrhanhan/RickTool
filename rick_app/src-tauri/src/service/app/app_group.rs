use rick_core::sqlite::{QueryDatabaseOperate, SqlWrapper, UpdateDatabaseOperate};
use crate::app::service::ServiceRegister;
use crate::global::{RickResult};
use crate::seq::{increase_table};
use crate::service::{common_result, convert_result};
use crate::store::app::AppGroup;
use crate::store::Id;


/// 服务注册
pub fn init_service(_register: &ServiceRegister) {
    _register.register_closure_fn("/app/group/save", save_app_group);
    _register.register_closure_fn("/app/group/update", update_app_group);
    _register.register_closure_fn("/app/group/delete", delete_app_group);
    _register.register_run_fn("/app/group/list", list_app_group);
}
/// 保存分组
fn save_app_group(mut app: AppGroup) -> RickResult<AppGroup> {
    app.id = increase_table::<AppGroup>();
    common_result(AppGroup::save(&app), app)
}
/// 保存分组
fn update_app_group(app: AppGroup) -> RickResult<AppGroup> {
    common_result(AppGroup::update_by_id(&app, app.id), app)
}
/// 保存分组
fn delete_app_group(id: Id) -> RickResult<usize> {
    convert_result(AppGroup::delete_by_id(id.id))
}

fn list_app_group() -> RickResult<Vec<AppGroup>> {
    convert_result(AppGroup::select_list(&SqlWrapper::new()))
}