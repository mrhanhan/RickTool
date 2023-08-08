use std::collections::HashMap;
use crate::app::module::ModuleAction;
use crate::app::service::{ServiceRegister, ServiceResult};
use crate::context::get_application;
use crate::modules::SqliteAction;

/// 初始化测试服务
pub fn init_test_service(register: &ServiceRegister) {
    register.register_fn("test", |invoke, _map: HashMap<String, String>|{
        let app = get_application();
        if let Some(_sqlite) = app.module_manager().get_module("sqlite") {
            _sqlite.action(ModuleAction::command_serialize(SqliteAction::Query.into(), "select * from user"));
        }
        invoke.resolver.resolve(ServiceResult::<i32>::success_data_message(200, "调用成功"))
    });
}