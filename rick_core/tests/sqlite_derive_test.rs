use rick_core::sqlite::{Connection, QueryDatabaseOperate, UpdateDatabaseOperate, SqlWrapper};
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
    println!("{:#?}", User::select_list(SqlWrapper::new().eq("id", 1)));
    // 保存
    let mut vec: Vec<User> = Vec::with_capacity(100);
    for i in 311..410 {
        vec.push(User {
            id: i,
            name: format!("测试用户: {}", i),
            password: format!("测试用户: {}", i)
        });
    }
    User::save_batch_vec(vec).expect("保存失败");
    println!("全部数据  {:#?}", User::select_list(&SqlWrapper::new()));
    User::delete_all().expect("删除失败");
    println!("全部数据  {:#?}", User::select_list(&SqlWrapper::new()));
    User::save(&User{id: 1, name: "admin".into(), password: "admin".into()}).expect("保存失败");
    User::save(&User{id: 2, name: "admin".into(), password: "admin".into()}).expect("保存失败");
    println!("全部数据  {:#?}", User::select_list(&SqlWrapper::new()));
    User::update().set("username", "Jenif").update(SqlWrapper::new().eq("id", 1))
        .expect("更新数据");
    println!("全部数据  {:#?}", User::select_list(&SqlWrapper::new()));
    println!("根据ID查询  {:#?}", User::select_by_id(1));

}