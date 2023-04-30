use tauri::{generate_handler, command, AppHandle};

use crate::{config::tauri::{TauriBuilder}, context::{use_application, api_manager::{ApiManager, self}}};


#[command(async)]
pub fn api(window: tauri::Window, handler: AppHandle, operate: String, data: String) ->  Result<String, String> {
    let mut context = api_manager::CallContext::new(&window, &handler, &data, &operate);
    use_application().call_api(&mut context)
}

pub fn register_handler(builder: TauriBuilder) -> TauriBuilder {
    builder.invoke_handler(generate_handler![api])
}