use std::fmt::format;
use std::fs::create_dir_all;
use tauri::api::path::{app_config_dir, app_data_dir, app_dir, executable_dir};
use crate::app::application::{Application, ApplicationEvent};
use crate::app::module::{Module, ModuleAction, ModuleActionManager, ModuleActionResult, ModuleError, ModuleMeta, ActionFunc};
use crate::context::get_application;
use crate::define_event;
use crate::model::AppLog;
use crate::model::LogLevel::INFO;
use crate::modules::sqlite_module::SqliteAction::{CreateTable, Execute, Query};
use crate::utils::ArcVal;

struct SqliteConfig {
    /// 数据库文件
    db_file: String,
}

#[derive(Clone)]
pub struct SqliteModule {
    _manager: ModuleActionManager,
    _config: ArcVal<SqliteConfig>,
}

define_event!(SqliteAction => Query, Execute, CreateTable);

impl SqliteModule {
    pub fn new() -> Self {
        Self {
            _manager: ModuleActionManager::new(),
            _config: ArcVal::new(SqliteConfig {
                db_file: String::new()
            }),
        }
    }

    /// 查询
    pub fn query(&self) -> Box<ActionFunc> {
        // 获取执行的SQL
        let _self = Clone::clone(self);
        Box::new(move |_action: ModuleAction| {
            println!("执行查询语句:{:?}", _action);
            if let Some(_sql) = _action.get::<String>() {
                let app = get_application();

                AppLog::module_simple(INFO, &_self,  format!("执行查询语句:{}", _self._config.db_file).as_str()).send_all(&app);
                println!("提交事件");
            }
            ModuleActionResult::success(None)
        })
    }
    /// 查询
    pub fn execute(&self) -> Box<ActionFunc> {
        // 获取执行的SQL
        Box::new(|_action: ModuleAction| {
            if let Some(_sql) = _action.get::<String>() {}
            ModuleActionResult::success(None)
        })
    }
    /// 查询
    pub fn create_time(&self) -> Box<ActionFunc> {
        // 获取执行的SQL
        Box::new(|_action: ModuleAction| {
            if let Some(_sql) = _action.get::<String>() {}
            ModuleActionResult::success(None)
        })
    }
}

impl Module for SqliteModule {
    fn meta(&self) -> ModuleMeta {
        ("sqlite", "sqlite 数据").into()
    }

    fn on_init(&self, _app: Application) -> Result<(), ModuleError> {
        let _app_config = _app.app_handler().config();
        self._config.using(|_config| {
            let buf = app_data_dir(&_app_config).unwrap();
            let data_dir = buf.join("data");
            if !data_dir.is_dir() || !data_dir.exists() {
                create_dir_all(&data_dir).unwrap();
            }
            _config.db_file = String::from(data_dir.join("app.db").to_str().unwrap());
        });
        // 添加操作
        self._manager.add_into(Query, self.query());
        self._manager.add_into(Execute, self.execute());
        self._manager.add_into(CreateTable, self.create_time());
        let _self = Clone::clone(self);
        Ok(())
    }


    fn on_install(&self, _app: Application) -> Result<(), ModuleError> {
        Ok(())
    }

    fn on_uninstall(&self, _app: Application) -> Result<(), ModuleError> {
        Ok(())
    }

    fn close(&self) {
        println!("sqlite 关闭");
    }

    fn action(&self, _action: ModuleAction) -> ModuleActionResult {
        self._manager.call(_action.cmd(), _action)
    }
    fn clone(&self) -> Box<dyn Module> {
        Box::new(Clone::clone(self))
    }
}
