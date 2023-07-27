use tauri::{App, AppHandle, Invoke, Wry};

pub type RickApp = App<Wry>;
pub type RickRuntime = Wry;
pub type RickInvoke = Invoke<RickRuntime>;
pub type RickAppHandler = AppHandle<Wry>;