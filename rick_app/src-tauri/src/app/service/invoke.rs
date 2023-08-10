use std::sync::{Arc, Mutex};
use serde::Serialize;
use serde_json::{to_value, Value};
use tauri::{StateManager, Window};
use crate::global::RickInvoke;

/// 服务调用
#[derive(Clone)]
pub struct ServiceInvoke {
    exception: Arc<Mutex<bool>>,
    data: Value,
    window: Window,
    state: Arc<StateManager>,
    value: Arc<Mutex<Option<Value>>>
}

impl ServiceInvoke {

    pub fn exception(&self) -> bool {
        *self.exception.lock().unwrap()
    }
    pub fn data(&self) -> Value {
        self.data.clone()
    }
    pub fn window(&self) -> Window {
        self.window.clone()
    }
    pub fn state(&self) -> Arc<StateManager> {
        self.state.clone()
    }
    pub fn value(&self) -> Arc<Mutex<Option<Value>>> {
        self.value.clone()
    }

    /// 结束值
    pub fn resolve<T: Serialize>(&self, value: T){
        *self.value.lock().unwrap() = Some(to_value(value).unwrap());
    }
    /// 结束值
    pub fn reject<T: Serialize>(&self, value: T){
        *self.exception.lock().unwrap() = true;
        *self.value.lock().unwrap() = Some(to_value(value).unwrap());
    }

    pub fn send(self, invoke: RickInvoke) {
        let exception = {*(&self).exception.lock().unwrap()};
        let value = {
            match *(&self).value.lock().unwrap() {
                None => {Value::Null}
                Some(ref _val) => _val.clone()
            }};
        if exception {
            invoke.resolver.reject(value);
        } else {
            invoke.resolver.resolve(value);
        }
    }
}

impl From<&RickInvoke> for ServiceInvoke {
    fn from(value: &RickInvoke) -> Self {
        ServiceInvoke {
            exception: Arc::new(Mutex::new(false)),
            data: value.message.payload().clone(),
            window: value.message.window(),
            state: value.message.state().clone(),
            value: Arc::new(Mutex::new(None))
        }
    }
}