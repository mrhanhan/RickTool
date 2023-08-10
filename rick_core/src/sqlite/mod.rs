mod query;
mod cond;
mod conn;
mod update;

use sqlite::State::Done;
use sqlite::Statement;
pub use conn::*;
pub use cond::*;
pub use query::*;
pub use update::*;

/// 表字段
#[derive(Debug)]
pub struct TableColumn {
    /// 字段
    pub field: ColumnValue,
    /// 列名称
    pub column: ColumnValue,
    /// 是否是ID 字段
    pub id: bool
}

/// 描述表的Table
pub trait Table {
    /// 获取表名称
    fn table_name() -> &'static str;
    /// 获取数据库连接
    fn conn() -> Connection;
    /// 字段内容
    fn columns() -> Vec<TableColumn>;
    /// 获取ID字段
    fn id_column() -> Option<ColumnValue> {
        for x in Self::columns() {
            if x.id {
                return Some(x.column);
            }
        }
        None
    }
}

/// 保存绑定
pub trait SaveBind: Table {
    /// 绑定字段
    fn bind(&self, statement: &mut Statement) -> Result<(), SqlError>;
    /// 开始
    fn bind_index(&self, statement: &mut Statement, start_index: usize) -> Result<(), SqlError>;
    /// 更新设置 更新值，include_id 是否包含对ID的设置
    fn update_set<T: Table>(&self, update: UpdateWrapper<T>, include_id: bool) ->  UpdateWrapper<T>;
}

impl<T: Table + From<sqlite::Row>> QueryDatabaseOperate for T {
    type Model = T;
}

impl<T: Table + SaveBind> UpdateDatabaseOperate for T {
    type Model = T;
}


fn done(mut _statement: Statement, connection: &Connection) -> Result<usize, SqlError>{
    loop {
        match _statement.next() {
            Ok(_state) => {
                if let Done = _state {
                    return Ok(connection.change_count())
                }
            }
            Err(_err) => {
                return Err(_err)
            }
        }
    }
}
