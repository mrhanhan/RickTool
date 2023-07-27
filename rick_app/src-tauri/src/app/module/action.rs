use std::str::FromStr;
use serde::de::{DeserializeOwned, IntoDeserializer};
use serde::Serialize;
use serde_json::{to_string, Value};

/// Module动作
pub struct ModuleAction<'a> {
    /// 调用命令
    command: &'a str,
    /// 参数
    payload: Option<Value>
}

impl<'a> ModuleAction<'a> {

    pub fn command(command: &str) -> Self {
        Self {
            command,
            payload: None
        }
    }
    pub fn command_payload(command: &str, payload: Value) -> Self {
        Self {
            command,
            payload: Some(payload)
        }
    }
    pub fn command_serialize<S: Serialize>(command: &str, payload: &S) -> Self {
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
            if let Ok(_data) = T::deserialize(_value.into_deserializer()) {
                Some(_data);
            }
        }
        None
    }
}
/// Command
impl<'a> Into<ModuleAction<'a>> for &'a str {
    fn into(self) -> ModuleAction<'a> {
        ModuleAction::command(self)
    }
}
/// Command
impl<'a> Into<ModuleAction<'a>> for (&'a str, Option<Value>) {
    fn into(self) -> ModuleAction<'a> {
        match self.1 {
            None => ModuleAction::command(self.0),
            Some(_payload) => ModuleAction::command_payload(self.0, _payload)
        }
    }
}
/// Command
impl<'a> Into<ModuleAction<'a>> for (&'a str, Value) {
    fn into(self) -> ModuleAction<'a> {
        ModuleAction::command_payload(self.0, self.1)
    }
}

/// Command
impl<'a, T: Serialize> Into<ModuleAction<'a>> for (&'a str, Option<&T>) {
    fn into(self) -> ModuleAction<'a> {
        match self.1 {
            None => ModuleAction::command(self.0),
            Some(_payload) => ModuleAction::command_serialize(self.0, _payload)
        }
    }
}

/// 响应值
pub struct ModuleResult(Result<Option<Value>, &'static str>);

impl ModuleResult {
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
            if let Ok(_data) = T::deserialize(_value.into_deserializer()) {
                Some(_data);
            }
        }
        None
    }
}
