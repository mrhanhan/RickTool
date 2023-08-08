use crate::app::application::Application;
use crate::app::module::{Module, ModuleAction, ModuleError, ModuleMeta};
use crate::modules::SqliteAction;

struct AppModule;


impl Module for AppModule {
    fn meta(&self) -> ModuleMeta {
        ("app", "App模块").into()
    }
    fn on_init(&self, app: Application) -> Result<(), ModuleError> {
        // 初始化App模块
        // 初始化数据库DDL
        self.init_db_ddl(app);
        Ok(())
    }
    fn clone(&self) -> Box<dyn Module> {
        Box::new(AppModule)
    }
}

const CONFIG_DDL: &str = r"
    create
";

impl AppModule {

    /// 初始化DDL
    fn init_db_ddl(&self, app: Application) {
        if let Some(_sqlite) = app.module_manager().get_module("sqlite") {
            _sqlite.action(ModuleAction::command_serialize(SqliteAction::Execute.into(), CONFIG_DDL));
        }
    }
}