use rick_core::sqlite::*;
use rick_core_macro::ITable;

pub fn hello() -> Connection {
    Connection::new(r"E:\Rick\RickTools\sqlite.db")
}

#[derive(ITable, Debug)]
#[table(table = "user", conn = "hello")]
struct User{
    /// ID
    #[column(id = true)]
    id: i64,
    /// 名称
    #[column(column="username")]
    name: String,
    /// 密码
    password: String
}
#[test]
fn test() {
    User::save(&User{
        id: 1,
        name: "Hello".into(),
        password: "Hello".into()
    });
    println!("{:#?}", User::select_list(SqlWrapper::new().eq("id", 1)));
    let conn = hello();
    conn.begin_transaction().expect("开启事务失败");
    User::delete_all_with_conn(&conn).expect("删除全部数据");
    conn.rollback().expect("回滚失败");
    println!("{:#?}", User::select_list(SqlWrapper::new().eq("id", 1)));
}