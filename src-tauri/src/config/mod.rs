pub(crate) mod tauri;


pub(crate) fn init_config() {
    tauri::init_tauri_config();
}