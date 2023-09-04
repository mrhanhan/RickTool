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

/// 需要运行的App 信息
#[derive(Serialize, Deserialize, ITable, Debug, Default)]
#[table(table = "v_app", conn = "crate::modules::app_db")]
pub struct App {
    /// ID
    #[column(id = true)]
    pub id: i32,
    /// 分组ID 默认为 0
    pub group_id: i32,
    /// 应用程序 名称
    pub name: String,
    /// 应用程序类型 100 可执行程序 200 JAVA程序 201 Python 程序 202 NodeJs 300 网页
    #[column(column = "type")]
    pub target_type: i32,
    /// 程序目录
    pub target: String,
    /// Logo 路径
    pub logo_path: String,
    /// 备注
    pub remark: String,
    /// 创建事件
    pub create_time: i32,
    /// 额外的字段
    #[column(exclude = true)]
    pub logo: Option<Vec<u8>>,
    /// 额外的字段
    #[column(exclude = true)]
    pub ext_vec: Option<Vec<AppExt>>,
    /// 额外的字段
    #[column(exclude = true)]
    pub start_vec: Option<Vec<AppStart>>
}

/// 应用程序扩展信息
#[derive(Serialize, Deserialize, ITable, Debug, Default)]
#[table(table = "v_app_ext", conn = "crate::modules::app_db")]
pub struct AppExt {
    /// ID
    #[column(id = true)]
    pub id: i32,
    /// APP ID
    pub app_id: i32,
    /// 启动方式ID
    pub start_id: i32,
    /// 类型
    #[column(column = "type")]
    pub ty: String,
    /// code
    pub code: String,
    /// 配置值
    pub value: String
}

/// APP 启动方式
#[derive(Serialize, Deserialize, ITable, Debug, Default)]
#[table(table = "v_app_start", conn = "crate::modules::app_db")]
pub struct AppStart {
    /// ID
    #[column(id = true)]
    pub id: i32,
    /// APP ID
    pub app_id: i32,
    /// 方式名称
    pub name: String,
    /// 备注
    pub remark: String,
    /// 参数
    #[column(exclude = true)]
    pub args: Option<Vec<AppStartArgs>>,
    /// 额外的字段
    #[column(exclude = true)]
    pub ext_vec: Option<Vec<AppExt>>
}


#[derive(Serialize, Deserialize, ITable, Debug, Default)]
#[table(table = "v_app_start_args", conn = "crate::modules::app_db")]
pub struct AppStartArgs {
    /// ID
    #[column(id = true)]
    pub id: i32,
    /// APP ID
    pub app_id: i32,
    /// 启动方式ID
    pub start_id: i32,
    /// 分组 ID
    pub group_id: i32,
    /// 类型 1 固定参数类型 2 文件参数类型 3 输入参数类型 4 多选参数类型
    #[column(column = "type")]
    pub ty: i32,
    /// 方式名称
    pub name: String,
    /// 默认值
    pub default_value: String,
    /// 配置
    pub config: String,
    /// 参数是否支持添加多个
    pub multiple: i32,
    /// 是否可选 0 必选 1  可选
    pub optional: i32,
    /// 备注
    pub remark: String
}