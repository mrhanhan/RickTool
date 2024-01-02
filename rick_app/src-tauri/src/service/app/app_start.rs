use rick_core::error::AppError;
use rick_core::sqlite::QueryDatabaseOperate;
use crate::app::service::ServiceRegister;
use crate::global::RickResult;
use crate::store::app::App;
use crate::store::Id;

pub fn init_service(_register: &ServiceRegister) {
    _register.register_closure_fn("/app/start", start_app);
}



/// ================================================================ [ api ] ==================================================================


fn start_app(id: Id) -> RickResult<String> {
    let app = App::select_by_id(id.id).unwrap();
    if app.is_none() {
        return Err(AppError::new(404, "App 不存在"))
    }
    Ok("".into())
}


/// ================================================================ [ private ] ==================================================================


fn build_command(app: App) {

}