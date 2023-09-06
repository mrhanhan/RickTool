use crate::global::{RickInvoke, RickInvokeMessage, RickInvokeResolver};
use rick_core::error::AppError;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_value, to_value, Value};
use std::sync::{Arc, Mutex};
use tauri::{StateManager, Window};

/// 服务调用
#[derive(Clone)]
pub struct ServiceInvoke {
    exception: Arc<Mutex<bool>>,
    data: Value,
    window: Window,
    state: Arc<StateManager>,
    value: Arc<Mutex<Option<Value>>>,
}

impl ServiceInvoke {
    pub fn exception(&self) -> bool {
        *self.exception.lock().unwrap()
    }
    pub fn data(&self) -> Value {
        self.data.clone()
    }

    pub fn get<T: DeserializeOwned>(&self) -> Result<T, AppError> {
        match from_value::<T>(self.data.clone()) {
            Ok(_data) => Ok(_data),
            Err(_err) => Err(AppError::new(500, _err.to_string())),
        }
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
    pub fn resolve<T: Serialize>(&self, value: T) {
        *self.value.lock().unwrap() = Some(to_value(value).unwrap());
    }
    /// 结束值
    pub fn reject<T: Serialize>(&self, value: T) {
        *self.exception.lock().unwrap() = true;
        *self.value.lock().unwrap() = Some(to_value(value).unwrap());
    }

    pub fn send(self, invoke: RickInvoke) {
        let exception = { *(&self).exception.lock().unwrap() };
        let value = {
            match *(&self).value.lock().unwrap() {
                None => Value::Null,
                Some(ref _val) => _val.clone(),
            }
        };
        if exception {
            invoke.resolver.reject(value);
        } else {
            invoke.resolver.resolve(value);
        }
    }
    pub fn send_resolver(self, resolver: RickInvokeResolver) {
        let exception = { *(&self).exception.lock().unwrap() };
        let value = {
            match *(&self).value.lock().unwrap() {
                None => Value::Null,
                Some(ref _val) => _val.clone(),
            }
        };
        if exception {
            resolver.reject(value);
        } else {
            resolver.resolve(value);
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
            value: Arc::new(Mutex::new(None)),
        }
    }
}

impl From<&RickInvokeMessage> for ServiceInvoke {
    fn from(message: &RickInvokeMessage) -> Self {
        ServiceInvoke {
            exception: Arc::new(Mutex::new(false)),
            data: message.payload().clone(),
            window: message.window(),
            state: message.state().clone(),
            value: Arc::new(Mutex::new(None)),
        }
    }
}
