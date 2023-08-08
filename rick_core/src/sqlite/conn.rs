use sqlite::Statement;

pub type SqlError = sqlite::Error;

/// Sql连接
pub struct Connection(sqlite::Connection);

impl Connection {

    pub fn new(url: &str) -> Self{
        Self(sqlite::Connection::open(url).unwrap())
    }

    /// Statement
    pub fn prepare<T: AsRef<str>>(&self, statement: T) -> Result<Statement, SqlError> {
        self.0.prepare(statement)
    }
    /// 执行
    pub fn execute<T: AsRef<str>>(&self, statement: T) -> Result<(), SqlError> {
        self.0.execute(statement)
    }
    
}

