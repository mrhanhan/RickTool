use std::collections::HashMap;
use std::marker::PhantomData;
use crate::sqlite::{ColumnValue, Connection, done, SaveBind, SqlError, SqlValue, SqlWrapper, Table};

pub trait UpdateDatabaseOperate {
    type Model: Table + SaveBind;

    /// 保存数据
    fn save(model: &Self::Model) -> Result<usize, SqlError> {
        let mut sql = format!("insert into {} (", Self::Model::table_name());
        let mut values = String::new();
        let columns = Self::Model::columns();
        for i in 0..columns.len() {
            if i > 0 {
                sql.push_str(", ");
                values.push_str(", ");
            }
            sql.push_str(columns.get(i).unwrap().column);
            values.push_str(format!(":{}", columns.get(i).unwrap().column).as_str());
        }
        sql.push_str(") values (");
        sql.push_str(values.as_str());
        sql.push_str(")");
        let conn = Self::Model::conn();
        let statement = conn.prepare(sql);
        match statement {
            Ok((mut _statement, _)) => {
                if let Err(_err) = model.bind(&mut _statement) {
                    return Err(_err);
                }
                done(_statement, &conn)
            }
            Err(_err) => Err(_err)
        }
    }
    /// 保存数据
    fn save_batch(models: &[&Self::Model]) -> Result<usize, SqlError> {
        let mut sql = format!("insert into {} (", Self::Model::table_name());
        let mut values = String::from("(");
        let columns = Self::Model::columns();
        for i in 0..columns.len() {
            if i > 0 {
                sql.push_str(", ");
                values.push_str(", ");
            }
            sql.push_str(columns.get(i).unwrap().column);
            values.push_str("?");
        }
        values.push_str(")");
        sql.push_str(") values ");
        for index in 0..models.len() {
            if index > 0 {
                sql.push_str(",");
            }
            sql.push_str(values.as_str());
        }
        let conn = Self::Model::conn();
        let statement = conn.prepare(sql);
        match statement {
            Ok((mut _statement, conn)) => {
                for index in 0..models.len() {
                    let module = models[index];
                    println!("开始索引:{}", (index * columns.len()) + 1 );
                    if let Err(_err) = module.bind_index(&mut _statement, (index * columns.len()) + 1) {
                        return Err(_err);
                    }
                }
                done(_statement, &conn)
            }
            Err(_err) => Err(_err)
        }
    }
    /// 批量保存
    fn save_batch_vec(models: Vec<Self::Model>) -> Result<usize, SqlError> {
        let mut vec: Vec<&Self::Model> = Vec::new();
        for x in models.iter() {
            vec.push(x);
        }
        Self::save_batch(vec.as_slice())
    }
    /// 批量保存
    fn save_batch_vec_ref(models: Vec<&Self::Model>) -> Result<usize, SqlError> {
        Self::save_batch(models.as_slice())
    }
    /// 删除数科
    fn delete(wrapper: &SqlWrapper) -> Result<usize, SqlError> {
        let sql = format!("delete from {}", Self::Model::table_name());
        match wrapper.process(sql, &Self::Model::conn()) {
            Ok((_statement, conn)) => {
                done(_statement, conn)
            }
            Err(_err) => {
                Err(_err)
            }
        }
    }
    /// 根据自定字段删除
    fn delete_by(column: ColumnValue, value: SqlValue) -> Result<usize, SqlError> {
        Self::delete(SqlWrapper::new().eq(column, value))
    }
    /// 删除所有字段
    fn delete_all() -> Result<usize, SqlError> {
        Self::delete(&SqlWrapper::new())
    }

    /// 更新Wrapper
    fn update() -> UpdateWrapper<Self::Model> {
        UpdateWrapper::new()
    }
    /// 根据ID修改
    fn update_by_id<I: Into<SqlValue>>(model: &Self::Model, value: I) -> Result<usize, SqlError> {
        if let None = Self::Model::id_column() {
            return Err(SqlError {code: None, message: Some("请配置ID字段: #[column(id = true)]".into())});
        }
        let update = Self::update();
        model.update_set(update, false)
            .update(SqlWrapper::new()
                .eq(Self::Model::id_column().unwrap(), value.into()))
    }
}

/// 更新操作
pub struct UpdateWrapper<T: Table> {
    /// 更新内容
    map: HashMap<ColumnValue, SqlValue>,
    _marker: PhantomData<T>
}

impl<T: Table > UpdateWrapper<T> {

    fn new() -> Self{
        Self {
            map: HashMap::new(),
            _marker: PhantomData
        }
    }
    /// 设置值
    pub fn set<I: Into<SqlValue>>(mut self, column: ColumnValue, value: I) -> Self {
        self.map.insert(column, value.into());
        self
    }
    /// 更新
    pub fn update(self, wrapper: &SqlWrapper) -> Result<usize, SqlError> {
        let mut sql = format!("update {}", T::table_name());
        let mut first = true;
        for (key, _) in &self.map {
            if first {
                sql.push_str(" set ");
                first = false;
            } else {
                sql.push_str(", ")
            }
            sql.push_str(key);
            sql.push_str(" = ?");
        }
        let mut cond_sql = String::new();
        let mut cond_index = 0usize;
        let mut index = 1usize;
        wrapper.process_cond_sql(&mut cond_sql, &mut cond_index);
        sql.push_str(" WHERE ");
        sql.push_str(cond_sql.as_str());
        cond_sql.clear();
        match T::conn().prepare(sql) {
            Ok((mut _statement, conn)) => {
                // 处理当前参数
                for (_, value) in self.map {
                    if let Err(_err) = _statement.bind((index, &value)) {
                        return Err(_err);
                    }
                    index = index + 1;
                }
                if let Err(_err) = wrapper.process_cond_value(&mut _statement, &mut index) {
                    return Err(_err);
                }
                done(_statement, conn)
            }
            Err(_err) => {
                Err(_err)
            }
        }
    }
}