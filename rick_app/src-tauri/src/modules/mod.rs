use crate::app::application::Application;
use crate::modules::sqlite_module::SqliteModule;

pub mod sqlite_module;

pub fn init_modules(app: Application) -> Application{
    let manager = app.module_manager();
    manager.add_module(Box::new(SqliteModule::new()));
    app
}