use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use serde::de::{DeserializeOwned};
use serde::{Serialize};
use serde_json::{from_value, to_string, Value};

/// Module动作
#[derive(Clone, Debug)]
pub struct ModuleAction<'a> {
    /// 调用命令
    command: &'a str,
    /// 参数
    payload: Option<Value>
}

impl<'a> ModuleAction<'a> {

    pub fn command(command: &'a str) -> Self {
        Self {
            command,
            payload: None
        }
    }
    pub fn command_payload(command: &'a str, payload: Value) -> Self {
        Self {
            command,
            payload: Some(payload)
        }
    }
    pub fn command_serialize<S: Serialize>(command: &'a str, payload: &S) -> Self {
        let mut _payload: Option<Value> = None;
        if let Ok(_value) = to_string(&payload) {
            _payload = Some(Value::from_str(_value.as_str()).unwrap());
        }
        Self {
            command,
            payload: _payload
        }
    }

    /// 获取数据
    pub fn get<T: DeserializeOwned>(&self) -> Option<T> {
        if let Some(_value) = self.payload.as_ref() {
            if let Ok(_data) = from_value::<T>(_value.clone()) {
                Some(_data);
            }
        }
        None
    }
}
impl<'a, T: Into<&'a str>> From<T> for ModuleAction<'a> {
    fn from(value: T) -> Self {
        ModuleAction::command(value.into())
    }
}




/// 响应值
pub struct ModuleActionResult(Result<Option<Value>, &'static str>);

impl ModuleActionResult {
    /// 成功
    pub fn success(value: Option<Value>) -> Self {
        Self(Ok(value))
    }
    /// 成功
    pub fn fail(reason: &'static str) -> Self {
        Self(Err(reason))
    }

    pub fn success_serialize<T: Serialize>(value: Option<T>) -> Self {
        if let Some(_data) = value {
            if let Ok(_value) = to_string(&_data) {
                return Self::success(Some(Value::from_str(_value.as_str()).unwrap()));
            }

        }
        Self::success(None)
    }
    /// 获取数据
    pub fn get<T: DeserializeOwned>(&self) -> Option<T> {
        if let Ok(Some(_value)) = self.0.as_ref() {
            if let Ok(_data) = from_value::<T>(_value.clone()) {
                Some(_data);
            }
        }
        None
    }
}

pub type ActionFunc = dyn Fn(ModuleAction) -> ModuleActionResult;

/// 动作管理器
#[derive(Clone)]
pub struct ModuleActionManager {
    /// 服务Map
    _action_map: Arc<RwLock<HashMap<&'static str, Box<ActionFunc>>>>
}

impl ModuleActionManager {

    pub fn new() -> Self {
        Self {
            _action_map: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    /// 注册服务
    pub fn add(&self, operate: &'static str, action: Box<ActionFunc>) {
        let mut map = self._action_map.write().unwrap();
        map.entry(operate).or_insert(action);
    }
    /// 注册服务
    pub fn add_into<I: Into<&'static str>>(&self, operate: I, action: Box<ActionFunc>) {
        self.add(operate.into(), action);
    }

    /// 注册服务
    pub fn call(&self, operate: &'static str, action: ModuleAction) -> ModuleActionResult {
        let mut map = self._action_map.read().unwrap();
        if let Some(_action_func) = map.get(operate) {
            return _action_func(action)
        }
        ModuleActionResult::fail("non")
    }

}