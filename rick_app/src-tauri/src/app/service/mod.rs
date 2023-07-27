use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::global::RickInvoke;

/// 结果
#[derive(Deserialize, Serialize)]
pub struct ServiceResult<'a, T: Serialize> {
    /// Code
    code: isize,
    message: &'a str,
    data: Option<T>
}

impl<'a, T: Serialize> ServiceResult<'a, T> {

    pub fn create(code: isize, data: Option<T>, message: &'a str) -> Self{
        Self {
            code, message, data
        }
    }

    pub fn success() -> Self{
        Self::create(200, None, "success")
    }

    pub fn success_data(data: T) -> Self{
        Self::create(200, Some(data), "success")
    }
    pub fn success_message(msg: &'a str) -> Self{
        Self::create(200, None, msg)
    }
    pub fn success_data_message(data: T, msg: &'a str) -> Self {
        Self::create(200, Some(data), msg)
    }

    pub fn fail() -> Self{
        Self::create(500, None, "fail")
    }

    pub fn fail_code(code: isize) -> Self{
        Self::create(code, None, "fail")
    }
    pub fn fail_reason(msg: &'a str) -> Self{
        Self::create(500, None, msg)
    }
    pub fn fail_data(data: T) -> Self{
        Self::create(500, Some(data), "fail")
    }
}


/// 服务
pub trait Service {
    /// 服务
    fn service(&self, action: RickInvoke);
}


struct ClosureFnService<F: Fn(RickInvoke)> {
    _func: F
}

impl<F: Fn(RickInvoke)> Service for ClosureFnService<F> {
    fn service(&self, action: RickInvoke) {
        let f = &self._func;
        f(action);
    }
}


/// 服务注册
#[derive(Clone, Default)]
pub struct ServiceRegister {
    /// 服务Map
    _service_map: Arc<RwLock<HashMap<String, Box<dyn Service>>>>,
    _sinks: Arc<RwLock<Vec<Box<ServiceSink>>>>
}

pub type ServiceSink = dyn Fn(RickInvoke) -> Result<(), RickInvoke>;

impl ServiceRegister {
    pub fn new() -> Self {
        Self {
            _service_map: Arc::new(RwLock::new(HashMap::new())),
            _sinks: Arc::new(RwLock::new(Vec::new()))
        }
    }
    pub fn add_sink(&self, sink: Box<dyn Fn(RickInvoke) -> Result<(), RickInvoke>>) {
        self._sinks.write().unwrap().push(Box::new(sink));
    }
    /// 注册服务
    pub fn register(&self, operate: String, service: Box<dyn Service>) {
        let mut map = self._service_map.write().unwrap();
        map.entry(operate).or_insert(service);
    }
    /// 注册闭包函数
    pub fn register_fn<F: Fn(RickInvoke, T) + Send + 'static, T: DeserializeOwned, O: Into<String>>(&self, operate: O, func: F) {
        let _c = move |invoke: RickInvoke| {
            let data = T::deserialize(invoke.message.payload());
            match data {
                Ok(_data) => {
                    func(invoke, _data);
                }
                Err(_err) => {
                    invoke.resolver.reject(ServiceResult::<i32>::fail_reason(format!("参数错误:{}", _err).as_str()))
                }
            }
        };
        let service_box: Box<dyn Service> = Box::new(ClosureFnService {_func: _c});
        self.register(operate.into(), service_box);
    }
    /// Sink 调用
    pub fn sink_call(&self, invoke: RickInvoke) -> Result<(), RickInvoke> {
        {
            let map = self._service_map.read().unwrap();
            if let Some(service) = map.get(invoke.message.command()) {
                service.service(invoke);
                return Ok(());
            }
        }
        let mut _invoke = invoke;
        {
            let sinks = self._sinks.read().unwrap();
            for sink in sinks.iter() {
                let result = sink(_invoke);
                match result {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(invoke) => {
                        _invoke = invoke;
                    }
                }
            }
        }
        Err(_invoke)
    }

    /// 调用服务
    pub fn call(&self, invoke: RickInvoke) {
        if let Err(_invoke) = self.sink_call(invoke) {
            _invoke.resolver.reject(ServiceResult::<i32>::fail_reason("服务不存在"));
        }
    }
}