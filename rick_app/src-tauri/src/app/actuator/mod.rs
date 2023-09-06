use crate::app::actuator::model::Action;
use std::sync::{Arc, Mutex};

mod model;
mod shell;

#[derive(Copy, Clone, Debug)]
pub enum DataType {
    Stdout,
    StdErr,
    Other,
}

/// 执行会话
pub trait ExecuteSession: Sync + Send {
    /// on_close 执行结束时候调用
    fn on_finish(&self, callback: Box<dyn Fn(&dyn ExecuteSession)>);
    /// 写入数据
    fn write(&self, data: &[u8]);
}

/// 默认实现得
pub(crate) struct DefaultExecuteSession {
    pub(crate) on_finish_callbacks: Arc<Mutex<Vec<Box<dyn Fn(&dyn ExecuteSession)>>>>,
    pub(crate) consumer_data: Arc<Option<Mutex<Box<dyn FnMut(&[u8])>>>>,
    pub(crate) finish_status: Arc<Mutex<bool>>,
}

impl DefaultExecuteSession {
    pub(crate) fn new() -> Self {
        Self {
            on_finish_callbacks: Arc::new(Mutex::new(Vec::new())),
            finish_status: Arc::new(Mutex::new(false)),
            consumer_data: Arc::new(Option::None),
        }
    }

    pub(crate) fn with_consumer(consumer: Box<dyn FnMut(&[u8])>) -> Self {
        let mut session = Self::new();
        session.consumer_data = Arc::new(Option::Some(Mutex::new(consumer)));
        session
    }

    pub(crate) fn set_consumer(&mut self, consumer: Box<dyn FnMut(&[u8])>) {
        self.consumer_data = Arc::new(Option::Some(Mutex::new(consumer)));
    }

    /// 结束执行
    pub(crate) fn finish(&self) {
        {
            match self.finish_status.lock() {
                Ok(mut _status) => {
                    *_status = true;
                }
                Err(mut _err) => {}
            }
        }
        let lock = self.on_finish_callbacks.lock();
        if let Ok(_vec) = lock {
            for i in 0.._vec.len() {
                let item = _vec.get(i);
                if let Some(_item) = item {
                    _item(self);
                }
            }
        }
    }
}

unsafe impl Sync for DefaultExecuteSession {}
unsafe impl Send for DefaultExecuteSession {}

impl ExecuteSession for DefaultExecuteSession {
    fn on_finish(&self, callback: Box<dyn Fn(&dyn ExecuteSession)>) {
        {
            if let Ok(_status) = self.finish_status.lock() {
                if *_status {
                    callback(self);
                    return;
                }
            }
        }
        if let Ok(mut _vec) = self.on_finish_callbacks.lock() {
            _vec.push(callback);
        }
    }
    fn write(&self, data: &[u8]) {
        if let Some(_fn) = self.consumer_data.as_ref() {
            if let Ok(mut __fn) = _fn.lock() {
                __fn(data);
            }
        }
    }
}
impl Clone for DefaultExecuteSession {
    fn clone(&self) -> Self {
        Self {
            on_finish_callbacks: self.on_finish_callbacks.clone(),
            finish_status: self.finish_status.clone(),
            consumer_data: self.consumer_data.clone(),
        }
    }
}

pub trait Executor<T: ExecuteSession, A: Action> {
    /// 执行Action
    fn execute(&self, action: &mut A) -> Result<T, ()>;
}
