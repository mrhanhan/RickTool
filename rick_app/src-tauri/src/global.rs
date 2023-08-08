use tauri::{App, AppHandle, Invoke, Window, Wry};
use rick_core::error::{AppError};

pub type RickApp = App<Wry>;
pub type RickRuntime = Wry;
pub type RickInvoke = Invoke<RickRuntime>;
pub type RickAppHandler = AppHandle<Wry>;
pub type RickWindow = Window<Wry>;
pub type RickResult<T> = Result<T, AppError>;