use std::fmt::{Display};
use darling::{FromMeta};
use syn::Type;

#[derive(Debug, FromMeta)]
pub struct TableMeta {
    ///
    #[darling(default)]
    pub table: Option<String>,
    #[darling(default)]
    pub conn: Option<String>
}

#[derive(Debug, FromMeta)]
pub struct TableFieldMeta {
    /// 字段
    #[darling(default)]
    pub column: Option<String>,
    /// 排除字段
    pub exclude: Option<bool>
}

#[derive(Debug, Clone)]
pub struct TableFieldInfo {
    /// 数据库字段
    pub column: String,
    /// 数据库字段
    pub field: String,
    /// 是否排除
    pub exclude: bool,
    /// 类型
    pub ty: Type
}
#[derive(Debug)]
pub struct TableInfo {
    /// 表名称
    pub table: String,
    /// 拦截函数
    pub conn: String,
    //// 字段
    pub fields: Vec<TableFieldInfo>
}