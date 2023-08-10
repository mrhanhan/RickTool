use std::collections::HashMap;
use std::fs::create_dir_all;
use tauri::api::path::{app_data_dir};
use rick_core::sqlite::{Connection, SqlValue};
use crate::app::application::{Application};
use crate::app::module::{Module, ModuleAction, ModuleActionManager, ModuleActionResult, ModuleError, ModuleMeta, ActionFunc};
use crate::{define_event, global_val, global_val_set};
use crate::utils::GlobalVal;
use crate::modules::sqlite_module::SqliteAction::{Execute, Query};

struct SqliteConfig {
    /// 数据库文件
    db_file: String,
}

global_val!(DB_CONFIG, SqliteConfig);


/// APP DB
pub fn app_db() -> Connection {
    let config = global_val!(DB_CONFIG);
    let db_file = &config.get_ref().unwrap().db_file;
    Connection::new(db_file.as_str())
}


#[derive(Clone)]
pub struct SqliteModule {
    _manager: ModuleActionManager
}

define_event!(SqliteAction => Query, Execute);

impl SqliteModule {

    pub fn new() -> Self {
        Self {
            _manager: ModuleActionManager::new()
        }
    }
    /// 查询
    pub fn query(&self) -> Box<ActionFunc> {
        // 获取执行的SQL
        let _self = Clone::clone(self);
        Box::new(move |_action: ModuleAction| {
            println!("执行查询语句:{:?}", _action);
            if let Some(_sql) = _action.get::<String>() {
                let app = app_db();
                 let statement = app.prepare(_sql);
                 if let Err(_e) = statement {
                     return ModuleActionResult::fail(_e);
                 }
                let _statement = statement.unwrap().0;
                let mut list: Vec<HashMap<String, SqlValue>> = Vec::new();
                let names: Vec<String> = _statement.column_names().iter().cloned().collect();
                for row in _statement {
                    if let Ok(_row) = row {
                        let mut map: HashMap<String, SqlValue> = HashMap::new();
                        for name in names.clone() {
                            let val = _row.read::<SqlValue, _>(name.as_str());
                            map.entry(name.clone()).or_insert(val);
                        }
                        list.push(map);
                    }
                }
                return ModuleActionResult::success_serialize(Some(list));
            }
            ModuleActionResult::success(None)
        })
    }
    /// 查询
    pub fn execute(&self) -> Box<ActionFunc> {
        // 获取执行的SQL
        Box::new(|_action: ModuleAction| {
            if let Some(_sql) = _action.get::<String>() {
                return app_db().execute(_sql).into()
            }
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

        let buf = app_data_dir(&_app_config).unwrap();
        let data_dir = buf.join("data");
        if !data_dir.is_dir() || !data_dir.exists() {
            create_dir_all(&data_dir).unwrap();
        }
        let db_file = String::from(data_dir.join("app.db").to_str().unwrap());
        println!("数据库文件:{}", &db_file);
        global_val_set!(DB_CONFIG, SqliteConfig {
            db_file: db_file
        });
        // 添加操作
        self._manager.add_into(Query, self.query());
        self._manager.add_into(Execute, self.execute());
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
