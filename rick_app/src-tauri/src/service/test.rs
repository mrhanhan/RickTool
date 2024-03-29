use crate::app::module::ModuleAction;
use crate::app::service::{ServiceRegister, ServiceResult};
use crate::context::get_application;
use crate::modules::SqliteAction;
use serde_json::Value;
use std::collections::HashMap;

/// 初始化测试服务
pub fn init_test_service(register: &ServiceRegister) {
    register.register_fn("test", |invoke, _map: HashMap<String, String>| {
        let app = get_application();
        if let Some(_sqlite) = app.module_manager().get_module("sqlite") {
            let result = _sqlite.action(ModuleAction::command_serialize(
                SqliteAction::Query.into(),
                "select * from user",
            ));
            invoke.resolve(ServiceResult::<Option<Value>>::success_data_message(
                result.get(),
                "成功",
            ))
        } else {
            invoke.resolve(ServiceResult::<i32>::success_data_message(400, "失败"))
        }
    });
}
