use crate::app::application::Application;
use crate::modules::sqlite_module::SqliteModule;

mod app_module;
mod sqlite_module;

pub use sqlite_module::{app_db, SqliteAction};

pub fn init_modules(app: Application) -> Application {
    let manager = app.module_manager();
    manager.add_module(Box::new(SqliteModule::new()));
    manager.add_module(Box::new(app_module::AppModule {}));
    app
}
