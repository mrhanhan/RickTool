use sqlite::Row;
use rick_core::sqlite::{Connection, DatabaseOperate, SqlWrapper, Table};
use rick_core_macro::Table;


pub fn hello() -> Connection {
    Connection::new(r"E:\Rick\RickTools\sqlite.db")
}

#[derive(Table, Debug)]
#[table(table = "user", conn = "hello")]
struct User{
    /// ID
    id: i64,
    /// 名称
    name: String,
    /// 密码
    password: String,
}

impl From<sqlite::Row> for User {
    fn from(value: Row) -> Self {
        User {
            id: value.read::<i64, _>("id"),
            name: String::new(),
            password: String::new()
        }
    }
}

#[test]
fn test() {
    println!("{:#?}", User::select_list(SqlWrapper::new()));
}