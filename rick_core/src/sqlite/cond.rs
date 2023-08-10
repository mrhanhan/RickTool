use sqlite::{BindableWithIndex, ParameterIndex, Statement, Value};
use crate::sqlite::cond::SqlCondEnum::{Group, In};
use crate::sqlite::cond::SqlCondJoin::{And, AndGroup, Or, OrGroup};
use crate::sqlite::{Connection, SqlError};
use crate::sqlite::SqlValue::{Float, Null, Number, Text};
use std::str::FromStr;
use serde::{Serialize, Serializer};

pub type ColumnValue = &'static str;

pub trait To<T> {
    /// 转换
    fn to(&self) -> T;
}

macro_rules! sql_value_impl {
    ($input: ty, Number) => {
        impl From<$input> for SqlValue {
            fn from(value: $input) -> Self {
                SqlValue::Number(value as i64)
            }
        }
        impl From<&$input> for SqlValue {
            fn from(value: &$input) -> Self {
                SqlValue::Number(*value as i64)
            }
        }

    };
    ($input: ty, Float) => {
        impl From<$input> for SqlValue {
            fn from(value: $input) -> Self {
                SqlValue::Float(value as f64)
            }
        }
        impl From<&$input> for SqlValue {
            fn from(value: &$input) -> Self {
                SqlValue::Float(*value as f64)
            }
        }

    }
}
macro_rules! sql_value_into_number {
    ($input: ty) => {
        impl From<&SqlValue> for $input {
            fn from(value: &SqlValue) -> Self {
                   match value {
                    Text(str) => Self::from_str(str.as_str()).unwrap(),
                    Float(value) => *value as $input,
                    Number(value) => *value as $input,
                    _ => 0 as $input
                }
            }
        }
        impl To<$input> for SqlValue {
            fn to(&self) -> $input {
                From::<&SqlValue>::from(self)
            }
        }

    };
}

#[derive(Clone, Debug)]
pub enum SqlValue {
    /// 数字
    Number(i64),
    /// Float
    Float(f64),
    /// 文本
    Text(String),
    /// 空
    Null
}

impl Serialize for SqlValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            Text(ref str) => serializer.serialize_str(str.as_str()),
            Float(ref value) => serializer.serialize_f64(*value),
            Number(ref value) => serializer.serialize_i64(*value),
            _ => serializer.serialize_none()
        }
    }
}

impl TryFrom<&Value> for SqlValue {
    type Error = SqlError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(match value {
            Value::Null => Null,
            Value::String(ref str) => SqlValue::Text(str.clone()),
            Value::Float(ref value) => SqlValue::Float(*value),
            Value::Integer(ref value) => SqlValue::Number(*value),
            _ => Null
        })
    }
}


impl From<&SqlValue> for String {
    fn from(value: &SqlValue) -> Self {
        match value {
            Text(str) => str.clone(),
            Float(value) => format!("{}", value),
            Number(value) => format!("{}", value),
            _ => String::new()
        }
    }
}
impl To<String> for SqlValue {
    fn to(&self) -> String {
        From::<&SqlValue>::from(self)
    }
}
sql_value_into_number!(u32);
sql_value_into_number!(i32);
sql_value_into_number!(usize);
sql_value_into_number!(isize);
sql_value_into_number!(u8);
sql_value_into_number!(i8);
sql_value_into_number!(u16);
sql_value_into_number!(i16);
sql_value_into_number!(u64);
sql_value_into_number!(i64);
sql_value_into_number!(f32);
sql_value_into_number!(f64);

impl<'a, T: From<&'a SqlValue>> To<Option<T>> for &'a SqlValue {
    fn to(&self) -> Option<T> {
        match &self {
            Null => None,
            _ => Some(T::from(self))
        }
    }
}

impl BindableWithIndex for &SqlValue {
    fn bind<T: ParameterIndex>(self, _statement: &mut Statement, _index: T) -> sqlite::Result<()> {
        match self.clone() {
            Number(_number) => {
                _number.bind(_statement, _index)
            }
            Float(_float) => {
                _float.bind(_statement, _index)
            }
            Text(_string) => {
                _string.as_str().bind(_statement, _index)
            }
            Null => {
                Option::<&str>::None.bind(_statement, _index)
            }
        }
    }
}

impl From<String> for SqlValue {
    fn from(value: String) -> Self {
        SqlValue::Text(value)
    }
}

impl From<&String> for SqlValue {
    fn from(value: &String) -> Self {
        SqlValue::Text(value.clone())
    }
}

impl From<&str> for SqlValue {
    fn from(value: &str) -> Self {
        SqlValue::Text(String::from(value))
    }
}

sql_value_impl!(u32, Number);
sql_value_impl!(i32, Number);
sql_value_impl!(usize, Number);
sql_value_impl!(isize, Number);
sql_value_impl!(u8, Number);
sql_value_impl!(i8, Number);
sql_value_impl!(u16, Number);
sql_value_impl!(i16, Number);
sql_value_impl!(u64, Number);
sql_value_impl!(i64, Number);
sql_value_impl!(f32, Float);
sql_value_impl!(f64, Float);

/// 枚举所有的连接条件
#[derive(Clone, Debug)]
pub enum SqlCondEnum {
    Equal(SqlValue),
    Like(SqlValue),
    In(Vec<SqlValue>),
    Lt(SqlValue),
    Le(SqlValue),
    Gt(SqlValue),
    Ge(SqlValue),
    Group(SqlWrapper),
}

/// 连接条件
#[derive(Clone, Debug)]
pub struct SqlCond {
    /// 数据库字段
    column: ColumnValue,
    /// 连接条件
    cond: SqlCondEnum,
    /// 连接逻辑
    join: SqlCondJoin,
}

/// 条件和条件之间的连接关系
#[derive(Copy, Clone, Debug)]
pub enum SqlCondJoin {
    And,
    Or,
    AndGroup,
    OrGroup,
}

/// Sql拼接
#[derive(Clone, Debug)]
pub struct SqlAppend {
    sql: String,
    args: Vec<SqlValue>,
}

/// Sql包装器
#[derive(Clone, Debug)]
pub struct SqlWrapper {
    /// Sql
    /// 当前状态下的连接逻辑
    current_join: SqlCondJoin,
    /// 条件
    cond: Vec<SqlCond>,
    /// 结束的Sql
    append_sql: Vec<SqlAppend>,
}

impl SqlWrapper {
    /// SqlWrapper
    pub fn new() -> Self {
        Self {
            current_join: And,
            cond: Vec::new(),
            append_sql: Vec::new(),
        }
    }
}

impl SqlWrapper {
    /// 添加条件
    pub fn apply_cond(&mut self, column: ColumnValue, cond: SqlCondEnum) -> &mut Self {
        self.cond.push(SqlCond { column, cond, join: self.current_join });
        self
    }

    pub fn eq<T: Into<SqlValue>>(&mut self, column: ColumnValue, cond: T) -> &mut Self {
        self.apply_cond(column, SqlCondEnum::Equal(cond.into()))
    }

    pub fn ge<T: Into<SqlValue>>(&mut self, column: ColumnValue, cond: T) -> &mut Self {
        self.apply_cond(column, SqlCondEnum::Ge(cond.into()))
    }
    pub fn gt<T: Into<SqlValue>>(&mut self, column: ColumnValue, cond: T) -> &mut Self {
        self.apply_cond(column, SqlCondEnum::Gt(cond.into()))
    }
    pub fn le<T: Into<SqlValue>>(&mut self, column: ColumnValue, cond: T) -> &mut Self {
        self.apply_cond(column, SqlCondEnum::Le(cond.into()))
    }
    pub fn lt<T: Into<SqlValue>>(&mut self, column: ColumnValue, cond: T) -> &mut Self {
        self.apply_cond(column, SqlCondEnum::Le(cond.into()))
    }
    pub fn like<T: Into<SqlValue>>(&mut self, column: ColumnValue, cond: T) -> &mut Self {
        self.apply_cond(column, SqlCondEnum::Like(cond.into()))
    }

    pub fn ins<T: Into<SqlValue>>(&mut self, column: ColumnValue, mut cond: Vec<T>) -> &mut Self {
        let mut vec: Vec<SqlValue> = Vec::with_capacity(cond.len());
        loop {
            if let Some(_val) = cond.pop() {
                vec.push(_val.into())
            }
            break;
        }
        self.apply_cond(column, In(vec))
    }

    pub fn and(&mut self) -> &mut Self {
        self.current_join = And;
        self
    }
    pub fn or(&mut self) -> &mut Self {
        self.current_join = Or;
        self
    }
    /// and_wrapper
    pub fn and_wrapper<F: Fn() -> SqlWrapper>(&mut self, func: F) -> &mut Self {
        self.cond.push(SqlCond { column: "", cond: Group(func()), join: AndGroup });
        self
    }
    /// and_wrapper
    pub fn or_wrapper<F: Fn() -> SqlWrapper>(&mut self, func: F) -> &mut Self {
        self.cond.push(SqlCond { column: "", cond: Group(func()), join: OrGroup });
        self
    }
    /// 拼接Sql
    pub fn append<F: AsRef<str>>(&mut self, sql: F) -> &mut Self {
        self.append_sql.push(SqlAppend { sql: String::from(sql.as_ref()), args: Vec::new() });
        self
    }
    /// 拼接Sql
    pub fn append_args<F: AsRef<str>>(&mut self, sql: F, args: Vec<SqlValue>) -> &mut Self {
        self.append_sql.push(SqlAppend { sql: String::from(sql.as_ref()), args });
        self
    }

    /// 连接
    pub fn process<'l>(&self, sql: String, conn: &'l Connection) -> Result<Statement<'l>, SqlError>{
        let mut cond_sql = String::new();
        let mut append_sql = String::new();
        let mut cond_index: usize = 0;
        let mut new_sql = String::from(&sql);
        // 获取最终Sql
        // 判断SQL中是否存在Where, 并且Where 后面是否存在条件
        // 存在条件的情况下
        if self.cond.len() > 0 {
            self.process_cond_sql(&mut cond_sql, &mut cond_index);
            self.process_append_sql(&mut append_sql, &mut cond_index);
            let where_index = sql.find("WHERE");
            if let Some(_where_index) = where_index {
                let mut has_char = false;
                let cond_str = String::from(&sql[(_where_index + 5)..sql.len()]);
                for _char in cond_str.chars() {
                    if _char != ' ' {
                        has_char = true;
                        break;
                    }
                }
                if has_char {
                    // 如果存在，拼接第一个参数的连接条件
                    let _first_cond = self.cond.get(0).unwrap();
                    new_sql.push_str(match _first_cond.join {
                        And => " AND ",
                        AndGroup => " AND ",
                        Or => " OR ",
                        OrGroup => " OR",
                    });
                    new_sql.push_str(" ");
                }
            } else {
                new_sql.push_str(" WHERE ");
            }
        }
        // 拼接最终Sql
        new_sql.push_str(cond_sql.as_str());
        new_sql.push_str(" ");
        new_sql.push_str(append_sql.as_str());
        cond_sql.clear();
        append_sql.clear();
        // 绑定数据
        let _statement = conn.prepare(new_sql);
        match _statement {
            Ok(mut _statement) => {
                // 绑定数据
                let mut index = 1usize;
                if let Err(_err) = self.process_cond_value(&mut _statement, &mut index) {
                    return Err(_err);
                }
                if let Err(_err) = self.process_append_args(&mut _statement, &mut index) {
                    return Err(_err);
                }
                Ok(_statement)
            }
            Err(_err) => Err(_err)
        }
    }

    pub(crate) fn process_cond_sql(&self, cond_sql: &mut String, index: &mut usize) {
        for _cond in self.cond.clone() {
            if *index > 0 {
                cond_sql.push_str(match _cond.join {
                    And => " AND ",
                    AndGroup => " AND ",
                    Or => " OR ",
                    OrGroup => " OR",
                })
            }
            if let Group(_wrapper) = _cond.cond.clone() {
                _wrapper.process_cond_sql(cond_sql, index);
            } else if let In(_vec) = _cond.cond.clone() {
                cond_sql.push_str(" ");
                // 添加字段
                cond_sql.push_str(_cond.column);
                // 添加条件
                cond_sql.push_str(get_cond_value(&_cond.cond));

                cond_sql.push_str("(");
                for _index in 0.._vec.len() {
                    if _index > 0 {
                        cond_sql.push_str(",");
                    }
                    cond_sql.push_str(" ?");
                    *index = *index + 1;
                }
                cond_sql.push_str(")");
            } else {
                cond_sql.push_str(" ");
                // 添加字段
                cond_sql.push_str(_cond.column);
                // 添加条件
                cond_sql.push_str(get_cond_value(&_cond.cond));
                cond_sql.push_str(" ?");
                *index = *index + 1;
            }
        }
    }

    pub(crate) fn process_cond_value(&self, _statement: &mut Statement, index: &mut usize) -> Result<(), SqlError>  {
        for _cond in self.cond.clone() {
            match _cond.cond.clone() {
                SqlCondEnum::Equal(_val) => {
                    if let Err(_err) = _statement.bind((*index, &_val)) {
                        return Err(_err);
                    }
                    *index = *index + 1;
                }
                SqlCondEnum::Like(_val) => {
                    if let Err(_err) = _statement.bind((*index, &_val)) {
                        return Err(_err);
                    }
                    *index = *index + 1;
                }
                In(_vec) => {
                    for _index in 0.._vec.len() {
                        if let Err(_err) = _statement.bind((*index, _vec.get(_index).unwrap())) {
                            return Err(_err);
                        }
                        *index = *index + 1;
                    }
                }
                SqlCondEnum::Lt(_val) => {
                    if let Err(_err) = _statement.bind((*index, &_val)) {
                        return Err(_err);
                    }
                    *index = *index + 1;
                }
                SqlCondEnum::Le(_val) => {
                    if let Err(_err) = _statement.bind((*index, &_val)) {
                        return Err(_err);
                    }
                    *index = *index + 1;
                }
                SqlCondEnum::Gt(_val) => {
                    if let Err(_err) = _statement.bind((*index, &_val)) {
                        return Err(_err);
                    }
                    *index = *index + 1;
                }
                SqlCondEnum::Ge(_val) => {
                    if let Err(_err) = _statement.bind((*index, &_val)) {
                        return Err(_err);
                    }
                    *index = *index + 1;
                }
                Group(_wrapper) => {
                    _wrapper.process_cond_value(_statement, index)?;
                }
            }
        }
        Ok(())
    }

    /// 处理追加的Sql
    fn process_append_sql(&self, cond_sql: &mut String, index: &mut usize) {
        for _append in self.append_sql.clone() {
            cond_sql.push_str(" ");
            cond_sql.push_str(_append.sql.as_str());
            *index = *index + _append.args.len();
        }
    }

    /// 处理追加的Sql
    fn process_append_args(&self, _statement: &mut Statement, index: &mut usize) -> Result<(), SqlError> {
        for _append in self.append_sql.clone() {
            for _arg in _append.args.clone() {
                if let Err(_err) = _statement.bind((*index, &_arg)) {
                    return Err(_err);
                }
                *index = *index + 1;
            }
        }
        Ok(())
    }

}

fn get_cond_value(cond_enum: &SqlCondEnum) -> ColumnValue {
    match cond_enum {
        SqlCondEnum::Equal(_) => " = ",
        SqlCondEnum::Like(_) => " LIKE ",
        In(_) => " IN ",
        SqlCondEnum::Lt(_) => " < ",
        SqlCondEnum::Le(_) => " <= ",
        SqlCondEnum::Gt(_) => " > ",
        SqlCondEnum::Ge(_) => " >= ",
        Group(_) => ""
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_char() {
        let sql = String::from("select a, b, 你好, 你再干啥, c, d WHERE a=1 and b=2");
        println!("{:?}", sql.find("WHERE"));
        println!("{:?}", &sql[sql.find("WHERE").unwrap()..sql.len()]);
        println!("{:?}", "你".len());
        let sql = String::from("你");
        let mut charts = sql.chars();
        // println!("{:?} {}", sql.len(), &sql[0..1]);
        println!("{}", charts.next().unwrap());
    }
}