use serde::{Deserialize, Serialize};
use rick_core_macro::ITable;
use rick_core::sqlite::*;

#[derive(Serialize, Deserialize, ITable, Debug)]
#[table(table = "v_app_group", conn = "crate::modules::app_db")]
pub struct AppGroup {
    // ID
    #[column(id = true)]
    pub id: i32,
    // 分组名称
    pub name: String,
    // icon
    pub icon: String
}


#[derive(Serialize, Deserialize, ITable, Debug, Default)]
#[table(table = "v_app_runtime_item", conn = "crate::modules::app_db")]
pub struct AppRuntimeItem {
    /// ID
    #[column(id = true)]
    pub id: i32,
    /// App 运行环境ID
    pub app_runtime_id: i32,
    /// 环境Key
    pub code: String,
    /// 环境变量值
    pub value: String
}

#[derive(Serialize, Deserialize, ITable, Debug, Default)]
#[table(table = "v_app_runtime", conn = "crate::modules::app_db")]
pub struct AppRuntime {
    /// ID
    #[column(id = true)]
    pub id: i32,
    /// 环境名称
    pub name: String,
    /// 环境描述
    pub description: String,
    /// 是否包含系统环境变量
    pub include_system: i32,
    /// 默认Items 数据
    #[column(exclude = true)]
    pub items: Option<Vec<AppRuntimeItem>>
}