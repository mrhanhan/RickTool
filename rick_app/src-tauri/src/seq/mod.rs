use rick_core::sqlite::{QueryDatabaseOperate, SqlWrapper, UpdateDatabaseOperate};
use rick_core_macro::ITable;
use crate::modules::app_db;

#[derive(Debug, ITable)]
#[table(table="t_sequence", conn="app_db")]
pub struct Seq {
    /// Code
    code: String,
    /// count
    count: i32,
}

/// 增加Code
pub fn increase_seq<A: AsRef<str>>(code: A) -> i32 {
    // 判断code 是否存在
    if let Some(mut _seq) = Seq::select_by_id(code.as_ref()).unwrap() {
        let seq = _seq.count + 1;
        Seq::update().set("count", seq).update(SqlWrapper::new()
            .eq("code", code.as_ref())).unwrap();
        return seq
    }
    Seq::save(&Seq {
        code: String::from(code.as_ref()),
        count: 0
    }).unwrap();
    0
}

const Tables: &[&str] = &["v_app_group"];
/// 初始化表的序号
pub fn init_db_table_seq() {
    let conn = Seq::conn();
    let mut vec = Vec::new();
    for table in Tables {
        {
            let statement = conn.prepare(format!("select max(id) c from {}", *table)).unwrap();
            for r in statement {
             if let Ok(_r) = r {
                 let v: SqlValue = _r.read("c");
                 vec.push(Seq {code: String::from(*table), count: v.to()});
             }
            }
        }
    }
    Seq::save_batch_vec(vec).unwrap();
}