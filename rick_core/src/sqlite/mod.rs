mod query;
mod cond;
mod conn;

pub use conn::*;
pub use cond::*;
pub use query::*;

/// 表字段
#[derive(Debug)]
pub struct TableColumn {
    /// 字段
    pub field: ColumnValue,
    /// 列名称
    pub column: ColumnValue
}

/// 描述表的Table
pub trait Table {
    /// 获取表名称
    fn table_name() -> &'static str;
    /// 获取数据库连接
    fn conn() -> Connection;
    /// 字段内容
    fn columns() -> Vec<TableColumn>;
}

/// 可查询的Trait
pub trait DatabaseOperate {
    /// 可查询的Module
    type Model: Table + From<sqlite::Row>;

    /// 查询List
    fn select_list(wrapper: SqlWrapper) -> Result<Vec<Self::Model>, SqlError> {
        // 获取连接
        let sql = format!("select * from {}", Self::Model::table_name());
        match wrapper.process(sql, &Self::Model::conn()) {
            Ok(_statement) => {
                let mut result: Vec<Self::Model> = Vec::new();
                for row in _statement {
                    if let Ok(_row) = row {
                        result.push(Self::Model::from(_row));
                    }
                }
                Ok(result)
            }
            Err(_err) => {
                Err(_err)
            }
        }
    }
}

impl<T: Table + From<sqlite::Row>> DatabaseOperate for T {
    type Model = T;
}