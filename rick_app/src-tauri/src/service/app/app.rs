use std::path::PathBuf;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tauri::api::path::app_data_dir;
use rick_core::sqlite::*;
use crate::app::service::ServiceRegister;
use crate::context::get_application;
use crate::global::RickResult;
use crate::seq::increase_table;
use crate::store::app::{App, AppExt};

/// ============================================ [ init ] ===========================================
pub fn init_service(_register: &ServiceRegister) {
    _register.register_closure_fn("/app/list", list_app);
}

/// ============================================ [ model ] ===========================================
#[derive(Serialize, Deserialize, Debug)]
struct AppListParams {
    /// 搜索关键字
    keyword: Option<String>,
    /// 查询分组
    group_id: Option<i32>
}

/// ============================================ [ api ] ===========================================


/// 查询列表
fn list_app(params: AppListParams) -> RickResult<Vec<App>> {
    let mut wrapper = SqlWrapper::new();
    let AppListParams {keyword, group_id} = params;
    if let Some(_keyword) = keyword {
        let _keyword = format!("%{}%", _keyword);
        wrapper.like("name", _keyword);
    }
    if let Some(_group_id) = group_id {
        wrapper.eq("group_id", _group_id);
    }
    match App::select_list(&wrapper) {
        Ok(mut _app_list) => {
            for _app in _app_list.as_mut_slice() {
                _app.logo = Some(get_logo_data(&_app.logo_path));
            }
            Ok(_app_list)
        },
        Err(ref _err) => {
            Err(_err.into())
        }
    }
}

/// 保存应用程序
fn save_app(mut app: App) -> RickResult<App> {
    let conn = Arc::new(App::conn());
    if let Err(ref _err) = conn.begin_transaction() {
        return Err(_err.into());
    }
    // 注册事件提交
    conn.register_callback();
    app.id = increase_table::<App>();
    // 保存app 扩展信息
    let _ = save_app_ext(app.id, 0, app.ext_vec.take(), &conn)?;
    Ok(app)
}


/// ============================================ [ utils ] ===========================================


fn save_app_ext(app_id: i32, start_id: i32, ext_vec: Option<Vec<AppExt>>, conn: &Connection) -> RickResult<()> {
    if let Some(mut _ext_vec) = ext_vec {
        for _ext in _ext_vec.as_mut_slice() {
            _ext.id = increase_table::<AppExt>();
            _ext.app_id = app_id;
            _ext.start_id = start_id;
        }
        return match AppExt::save_batch_vec_with_conn(_ext_vec, conn) {
            Ok(_) => Ok(()),
            Err(ref _err) => Err(_err.into()),
        };
    }
    Ok(())
}


/// 获取logo 目录
pub fn get_logo_dir() -> PathBuf {
    let config = get_application().app_handler().config();
    let data_dir = app_data_dir(&config).unwrap();
    let logo_dir = data_dir.join("app").join("logo");
    if !logo_dir.exists() {
        std::fs::create_dir_all(&logo_dir).unwrap();
    }
    logo_dir
}
/// 获取Logo 数据
pub fn get_logo_data(name: &String) -> Vec<u8> {
    let logo_path = get_logo_dir().join(name.as_str());
    match std::fs::read(logo_path) {
        Ok(_data) => _data,
        Err(_) => vec![]
    }
}