
/// 异常数据
pub trait RickError {
    /// 获取异常码
    fn code(&self) -> isize;
    /// 获取异常消息
    fn message(&self) -> String;
}

impl RickError for sqlite::Error {
    fn code(&self) -> isize {
        match self.code {
            None => 0,
            Some(_code) => _code
        }
    }
    fn message(&self) -> String {
        match self.message {
            None => String::new(),
            Some(ref _code) => _code.clone()
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
