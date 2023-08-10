use serde::{Deserialize, Serialize};
use rick_core_macro::ITable;


#[derive(Serialize, Deserialize, ITable, Debug)]
#[table(table = "v_app_group", conn = "crate::modules::app_db")]
pub struct AppGroup {
    // ID
    #[column(id = true)]
    pub id: i32,
    // 分组名称
    pub name: String,
    // icon
    pub icon: String
}
