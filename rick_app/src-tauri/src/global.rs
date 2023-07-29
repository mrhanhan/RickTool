use tauri::{App, AppHandle, Invoke, Window, Wry};

pub type RickApp = App<Wry>;
pub type RickRuntime = Wry;
pub type RickInvoke = Invoke<RickRuntime>;
pub type RickAppHandler = AppHandle<Wry>;
pub type RickWindow = Window<Wry>;