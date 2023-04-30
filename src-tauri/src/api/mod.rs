use serde::{Serialize, Deserialize};

use crate::context::{api_manager::{CallContext, ApiManager}, use_application, api_handler::api_json};



fn hello(_ctx: &mut CallContext) -> Result<String, String> {
    println!("Hello 你好啊");
    Ok("word".into())
}

#[derive(Serialize, Deserialize)]
pub struct JsonHello {
    data: String
}

fn json_hello(_ctx: &mut CallContext, _json: JsonHello) -> Result<JsonHello, String> {

    println!("Hello a");
    Ok(_json)
}

pub(crate) fn register_api() {
    let app = use_application();
    // 注册API
    app.register_api("hello".into(), Box::new(hello)).unwrap();
    app.register_api("hello_json".into(), api_json(Box::new(json_hello))).unwrap();
}