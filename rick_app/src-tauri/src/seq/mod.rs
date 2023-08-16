use rick_core::sqlite::*;
use rick_core_macro::ITable;
use crate::modules::app_db;

#[derive(Debug, ITable)]
#[table(table="t_sequence", conn="app_db")]
pub struct Seq {
    /// Code
    #[column(id = true)]
    code: String,
    /// count
    #[column(column = "seq", id = true)]
    count: i32,
}


pub fn increase_table<T: Table>() -> i32 {
    let code = T::table_name();
    // 判断code 是否存在
    if let Some(mut _seq) = Seq::select_by_id(code).unwrap() {
        let seq = _seq.count + 1;
        Seq::update().set("seq", seq).update(SqlWrapper::new()
            .eq("code", code)).unwrap();
        return seq
    }
    Seq::save(&Seq {
        code: String::from(code),
        count: get_table_max_id(code, T::id_column().unwrap())
    }).unwrap();
    0
}

pub fn get_table_max_id(table_name: &str, id_column: &str) -> i32 {
    let conn = Seq::conn();
    let statement = conn.prepare(format!("select max({}) c from {}", id_column, table_name)).unwrap();
    for r in statement.0 {
        if let Ok(_r) = r {
            let v: SqlValue = _r.read("c");
            return v.to();
        }
    }
    return 0;
}