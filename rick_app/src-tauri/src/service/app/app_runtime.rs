use rick_core::error::AppError;
use rick_core::sqlite::{QueryDatabaseOperate, SqlWrapper, Table, UpdateDatabaseOperate};
use crate::app::service::ServiceRegister;
use crate::global::RickResult;
use crate::service::{common_result, convert_result};
use crate::store::app::{AppRuntime, AppRuntimeItem};
use crate::store::Id;

/// 服务注册
pub fn init_service(_register: &ServiceRegister) {
    _register.register_run_fn("/app/runtime/list", list_app_runtime);
    _register.register_closure_fn("/app/runtime/save", save_app_runtime);
    _register.register_closure_fn("/app/runtime/update", update_app_runtime);
    _register.register_closure_fn("/app/runtime/detail", detail_app_runtime);
    _register.register_closure_fn("/app/runtime/delete", delete_app_runtime);
}

fn list_app_runtime() -> RickResult<Vec<AppRuntime>> {
    match AppRuntime::select_list(&SqlWrapper::new()) {
        Ok(mut _list) => {
            Ok(_list)
        },
        Err(ref _err) => Err(_err.into())
    }
}

fn detail_app_runtime(id: Id) -> RickResult<AppRuntime> {
    match AppRuntime::select_by_id(id.id) {
        Ok(Some(mut _app_runtime)) => {
            let items = AppRuntimeItem::select_list(
                SqlWrapper::new().eq("app_runtime_id", id.id));
            if let Err(ref _err) = items {
                return Err(_err.into());
            }
            _app_runtime.items = Some(items.unwrap());
            Ok(_app_runtime)
        }
        Ok(None) => Err(AppError::new(404, "记录不存在")),
        Err(ref _err) => Err(_err.into())
    }
}



fn delete_app_runtime(id: Id) -> RickResult<usize> {
    match AppRuntime::delete_by_id(id.id) {
        Ok(_usize) => {
            if _usize <= 0 {
                return Err(AppError::new(404, "记录不存在"));
            }
            convert_result(AppRuntimeItem::delete_by("app_runtime_id", id.id.into()))
        },
        Err(ref _err) => Err(_err.into())
    }
}

fn save_app_runtime(mut data: AppRuntime) -> RickResult<AppRuntime> {
    data.id = crate::seq::increase_table::<AppRuntime>();
    // 保存Item
    if let Some(items) = data.items.as_mut() {
        for item in items {
            item.id = crate::seq::increase_table::<AppRuntimeItem>();
            item.app_runtime_id = data.id;
        }
    }
    // 保存记录
    if let Some(items) = data.items.take() {
        if items.len() > 0 {
            if let Err(ref _err) = AppRuntimeItem::save_batch(items.as_slice()) {
                return Err(_err.into());
            }
        }
        data.items = Some(items);
    }
    common_result(AppRuntime::save(&data), data)
}


fn update_app_runtime(mut data: AppRuntime) -> RickResult<AppRuntime> {

     match AppRuntime::update_by_id(&data, data.id) {
         Ok(_count) => {
            if _count <= 0 {
                return Err(AppError::new(404, "App运行环境不存在"))
            }
             // 保存Item
             if let Some(items) = data.items.as_mut() {
                 for item in items {
                     item.id = crate::seq::increase_table::<AppRuntimeItem>();
                     item.app_runtime_id = data.id;
                 }
             }
             if let Err(ref _err) = AppRuntimeItem::delete(SqlWrapper::new().eq("app_runtime_id", data.id)) {
                 return Err(AppError::new(500, "环境变量删除失败"))
             }
             if let Some(items) = data.items.as_ref() {
                 if items.len() > 0 {
                     if let Err(ref _err) = AppRuntimeItem::save_batch(items.as_slice()) {
                         return Err(_err.into());
                     }
                 }
             }
             Ok(data)
         },
         Err(ref _err) => Err(_err.into())
     }
}