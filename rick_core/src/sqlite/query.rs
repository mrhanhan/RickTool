use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use crate::sqlite::{Connection, SqlError, SqlValue, SqlWrapper, Table};

pub struct Query<T: Table> {
    _mark: PhantomData<T>,
}


/// 可查询的Trait
pub trait QueryDatabaseOperate {
    /// 可查询的Module
    type Model: Table + From<sqlite::Row>;

    fn select_list_with_conn(wrapper: &SqlWrapper, conn: &Connection) -> Result<Vec<Self::Model>, SqlError> {
        let sql = format!("select * from {}", Self::Model::table_name());
        match wrapper.process(sql, conn) {
            Ok((_statement, _)) => {
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
    /// 查询List
    fn select_list(wrapper: &SqlWrapper) -> Result<Vec<Self::Model>, SqlError> {
        // 获取连接
        Self::select_list_with_conn(wrapper, &Self::Model::conn())
    }

    fn select_by_id_with_conn<T: Into<SqlValue>>(value: T, conn: &Connection) -> Result<Option<Self::Model>, SqlError> {
        if let None = Self::Model::id_column() {
            return Err(SqlError {code: None, message: Some("请配置ID字段: #[column(id = true)]".into())});
        }
        let mut wrapper = SqlWrapper::new();
        wrapper.eq(Self::Model::id_column().unwrap(), value.into());
        match Self::select_list_with_conn(&wrapper, conn) {
            Ok(mut _list) => {
                if _list.is_empty() {
                    return Ok(None)
                } else {
                    return Ok(_list.pop());
                }
            }
            Err(_err) => Err(_err)
        }
    }

    /// 根据ID 查询记录
    fn select_by_id<T: Into<SqlValue>>(value: T) -> Result<Option<Self::Model>, SqlError> {
        Self::select_by_id_with_conn(value, &Self::Model::conn())
    }

    fn select_map_with_conn<K: Eq + Hash>(wrapper: &SqlWrapper, key_func: fn(&Self::Model) -> K, conn: &Connection) -> Result<HashMap<K, Self::Model>, SqlError> {
        let vec = Self::select_list_with_conn(wrapper, conn);
        match vec {
            Ok(_list) => {
                let mut map = HashMap::new();
                for i in _list {
                    map.entry(key_func(&i))
                        .or_insert(i);
                }
                Ok(map)
            }
            Err(_err) => Err(_err)
        }
    }
    fn select_map<K: Eq + Hash>(wrapper: &SqlWrapper, key_func: fn(&Self::Model) -> K) -> Result<HashMap<K, Self::Model>, SqlError> {
        Self::select_map_with_conn(wrapper, key_func,  &Self::Model::conn())
    }
}
