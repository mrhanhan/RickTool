use crate::utils::thread_callback::{add_callback, ThreadExecuteStatus};
use sqlite::Statement;
use std::sync::Arc;

pub type SqlError = sqlite::Error;

/// Sql连接
pub struct Connection(sqlite::Connection);

impl Connection {
    pub fn new(url: &str) -> Self {
        Self(sqlite::Connection::open(url).unwrap())
    }

    /// Statement
    pub fn prepare<T: AsRef<str>>(&self, statement: T) -> Result<(Statement, &Self), SqlError> {
        self.0.prepare(statement).map(|f| (f, self))
    }
    /// 执行
    pub fn execute<T: AsRef<str>>(&self, statement: T) -> Result<(), SqlError> {
        self.0.execute(statement)
    }
    pub fn begin_transaction(&self) -> Result<(), SqlError> {
        self.0.execute("BEGIN TRANSACTION").into()
    }
    pub fn commit(&self) -> Result<(), SqlError> {
        self.0.execute("COMMIT").into()
    }
    pub fn rollback(&self) -> Result<(), SqlError> {
        self.0.execute("ROLLBACK").into()
    }
    pub fn change_count(&self) -> usize {
        self.0.change_count()
    }
}

pub trait ConnectionContext {
    fn register_callback(&self);
}
impl ConnectionContext for Arc<Connection> {
    fn register_callback(&self) {
        let conn = self.clone();
        add_callback(Box::new(
            move |_status: &ThreadExecuteStatus| match _status {
                ThreadExecuteStatus::Ok => {
                    conn.commit();
                }
                ThreadExecuteStatus::Panic => {
                    conn.rollback();
                }
            },
        ));
    }
}
