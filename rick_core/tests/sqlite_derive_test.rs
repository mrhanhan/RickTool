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
#[derive(ITable, Debug, Default)]
#[table(table = "v_app_runtime_item", conn = "hello")]
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

#[derive(ITable, Debug, Default)]
#[table(table = "v_app_runtime", conn = "hello")]
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

#[test]
fn test() {
    println!("{:#?}", User::select_list(SqlWrapper::new().eq("id", 1)));
    let a = Option::<Vec<AppRuntimeItem>>::default();
    // // 保存
    // let mut vec: Vec<User> = Vec::with_capacity(100);
    // for i in 311..410 {
    //     vec.push(User {
    //         id: i,
    //         name: format!("测试用户: {}", i),
    //         password: format!("测试用户: {}", i)
    //     });
    // }
    // User::save_batch_vec(vec).expect("保存失败");
    // println!("全部数据  {:#?}", User::select_list(&SqlWrapper::new()));
    // User::delete_all().expect("删除失败");
    // println!("全部数据  {:#?}", User::select_list(&SqlWrapper::new()));
    // User::save(&User{id: 1, name: "admin".into(), password: "admin".into()}).expect("保存失败");
    // User::save(&User{id: 2, name: "admin".into(), password: "admin".into()}).expect("保存失败");
    // println!("全部数据  {:#?}", User::select_list(&SqlWrapper::new()));
    // User::update().set("username", "Jenif").update(SqlWrapper::new().eq("id", 1))
    //     .expect("更新数据");
    // println!("全部数据  {:#?}", User::select_list(&SqlWrapper::new()));
    // println!("根据ID查询  {:#?}", User::select_by_id(1));

}