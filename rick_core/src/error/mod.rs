use tauri::App;
use crate::sqlite::SqlError;

/// 异常数据
pub trait RickError {
    /// 获取异常码
    fn code(&self) -> isize;
    /// 获取异常消息
    fn message(&self) -> String;
}

impl RickError for SqlError {
    fn code(&self) -> isize {
        match self.code.as_ref() {
            Some(_code) => *_code,
            None => 0
        }
    }
    fn message(&self) -> String {
        match self.message {
            Some(ref _str) => _str.clone(),
            None => String::new()
        }
    }
}

#[derive(Debug)]
pub struct AppError {
    code: isize,
    message: String
}

impl AppError {
    pub fn new<A: AsRef<str>>(code: isize, msg: A) -> Self {
        Self {
            code,
            message: String::from(msg.as_ref())
        }
    }
}

impl RickError for AppError {
    fn code(&self) -> isize {
        self.code
    }
    fn message(&self) -> String {
        self.message.clone()
    }
}

impl<E: RickError> From<&E> for AppError {
    fn from(value: &E) -> Self {
        Self {
            code: value.code(),
            message: value.message(),
        }
    }
}