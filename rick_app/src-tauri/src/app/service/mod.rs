mod invoke;

use std::collections::HashMap;
use std::ops::Deref;
use std::panic::{catch_unwind, UnwindSafe};
use std::sync::{Arc, RwLock};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use rick_core::error::{AppError, RickError};
pub use crate::app::service::invoke::ServiceInvoke;
use crate::context::get_application;
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
    fn service(&self, action: ServiceInvoke);
}


pub struct ClosureFnService<F> {
    _func: F
}

impl<F> ClosureFnService<F> {
    pub fn new(_func: F) -> Box<Self> {
        Box::new(Self {
            _func
        })
    }
}

impl<F: Fn(ServiceInvoke)> Service for ClosureFnService<F> {
    fn service(&self, action: ServiceInvoke) {
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

pub type ServiceSink = dyn Fn(String, ServiceInvoke) -> Result<(), AppError>;

impl ServiceRegister {
    pub fn new() -> Self {
        Self {
            _service_map: Arc::new(RwLock::new(HashMap::new())),
            _sinks: Arc::new(RwLock::new(Vec::new()))
        }
    }
    pub fn add_sink(&self, sink: Box<dyn Fn(String, ServiceInvoke) -> Result<(), AppError>>) {
        self._sinks.write().unwrap().push(Box::new(sink));
    }
    /// 注册服务
    pub fn register(&self, operate: String, service: Box<dyn Service>) {
        let mut map = self._service_map.write().unwrap();
        map.entry(operate).or_insert(service);
    }
    /// 注册闭包函数
    pub fn register_fn<F: Fn(ServiceInvoke, T) + Send + 'static, T: DeserializeOwned, O: Into<String>>(&self, operate: O, func: F) {
        let _c = move |invoke: ServiceInvoke| {
            let data = T::deserialize(invoke.data());
            match data {
                Ok(_data) => {
                    func(invoke, _data);
                }
                Err(_err) => {
                    invoke.reject(ServiceResult::<i32>::fail_reason(format!("参数错误:{}", _err).as_str()))
                }
            }
        };
        let service_box: Box<dyn Service> = Box::new(ClosureFnService {_func: _c});
        self.register(operate.into(), service_box);
    }

    pub fn register_closure_fn<M, E, F, T: DeserializeOwned, O: Into<String>>(&self, operate: O, func: F)
        where M: Serialize, E: RickError + 'static, F: (Fn(T) -> Result<M, E>) + Send + 'static
    {
        let _c = move |invoke: ServiceInvoke| {
            let data = T::deserialize(invoke.data());
            match data {
                Ok(_data) => {
                    match func(_data) {
                        Ok(_result_data) => {
                            invoke.resolve(ServiceResult::success_data(_result_data))
                        }
                        Err(_err) => {
                            invoke.reject(ServiceResult::<i32>::create(_err.code(), None, _err.message().as_str()));
                        }
                    }
                }
                Err(_err) => {
                    invoke.reject(ServiceResult::<i32>::fail_reason(format!("参数错误:{}", _err).as_str()))
                }
            }
        };
        let service_box: Box<dyn Service> = Box::new(ClosureFnService {_func: _c});
        self.register(operate.into(), service_box);
    }

    pub fn register_run_fn<M, E, F, O: Into<String>>(&self, operate: O, func: F)
        where M: Serialize, E: RickError + 'static, F: (Fn() -> Result<M, E>) + Send + 'static
    {
        let _c = move |invoke: ServiceInvoke| {
            match func() {
                Ok(_result_data) => {
                    invoke.resolve(ServiceResult::success_data(_result_data))
                }
                Err(_err) => {
                    invoke.reject(ServiceResult::<i32>::create(_err.code(), None, _err.message().as_str()));
                }
            }
        };
        let service_box: Box<dyn Service> = Box::new(ClosureFnService {_func: _c});
        self.register(operate.into(), service_box);
    }

    pub fn register_invoke_fn<M, E, F, O: Into<String>>(&self, operate: O, func: F)
        where M: Serialize, E: RickError + 'static, F: (Fn(&ServiceInvoke) -> Result<M, E>) + Send + 'static
    {
        let _c = move |invoke: ServiceInvoke| {
            match func(&invoke) {
                Ok(_result_data) => {
                    invoke.resolve(ServiceResult::success_data(_result_data))
                }
                Err(_err) => {
                    invoke.reject(ServiceResult::<i32>::create(_err.code(), None, _err.message().as_str()));
                }
            }
        };
        let service_box: Box<dyn Service> = Box::new(ClosureFnService {_func: _c});
        self.register(operate.into(), service_box);
    }

    /// 注册闭包函数
    pub fn register_unit_fn<M, E, F, T: DeserializeOwned, O: Into<String>>(&self, operate: O, func: F)
    where M: Serialize, E: RickError + 'static, F: (Fn(&ServiceInvoke, T) -> Result<M, E>) + Send + 'static
    {
        let _c = move |invoke: ServiceInvoke| {
            let data = T::deserialize(invoke.data());
            match data {
                Ok(_data) => {
                    match func(&invoke, _data) {
                        Ok(_result_data) => {
                            invoke.resolve(ServiceResult::success_data(_result_data))
                        }
                        Err(_err) => {
                            invoke.reject(ServiceResult::<i32>::create(_err.code(), None, _err.message().as_str()));
                        }
                    }
                }
                Err(_err) => {
                    invoke.reject(ServiceResult::<i32>::fail_reason(format!("参数错误:{}", _err).as_str()))
                }
            }
        };
        let service_box: Box<dyn Service> = Box::new(ClosureFnService {_func: _c});
        self.register(operate.into(), service_box);
    }
    /// Sink 调用
    pub fn sink_call(&self, command: String, invoke: ServiceInvoke) -> Result<(), AppError> {
        {
            let map = self._service_map.read().unwrap();
            if let Some(service) = map.get(command.as_str()) {
                service.service(invoke);
                return Ok(());
            }
        }
        {
            let sinks = self._sinks.read().unwrap();
            for sink in sinks.iter() {
                let result = sink(command.clone(), invoke.clone());
                match result {
                    Ok(_) => {
                        return Ok(());
                    }
                    _ => {

                    }
                }
            }
        }
        invoke.reject(ServiceResult::<i32>::fail_code(404));
        Ok(())
    }

    /// 异步调用
    pub fn async_call(&self, invoke: RickInvoke) {
        let app = get_application();
        let RickInvoke {message, resolver} = invoke;
        let this = self.clone();
        app.thread_pool().submit(Box::new(move || {
            let service_invoke = ServiceInvoke::from(&message);
            let msg: String = message.command().into();
            let _self = Arc::new(&this);
            let _service_ptr = &service_invoke as *const ServiceInvoke as usize;

            println!("{:#?}", service_invoke.data());
            let result = catch_unwind(move || {
                let _service = unsafe {&*(_service_ptr as *const ServiceInvoke)};
                _self.sink_call(msg, _service.clone()).unwrap()
            });

            match result {
                Ok(_) => {
                    service_invoke.send_resolver(resolver);
                }
                Err(_err) => {
                    resolver.reject(ServiceResult::<i32>::fail_code(500));
                }
            }
        })).expect("");
    }

    /// 调用服务
    pub fn call(&self, invoke: RickInvoke) {
        let service_invoke = ServiceInvoke::from(&invoke);
        let msg: String = invoke.message.command().into();
        let _self = Arc::new(self);
        let _service_ptr = &service_invoke as *const ServiceInvoke as usize;

        println!("{:#?}", service_invoke.data());
        let result = catch_unwind(move || {
            let _service = unsafe {&*(_service_ptr as *const ServiceInvoke)};
            self.sink_call(msg, _service.clone()).unwrap()
        });

        match result {
            Ok(_) => {
                service_invoke.send(invoke);
            }
            Err(_err) => {
                invoke.resolver.reject(ServiceResult::<i32>::fail_code(500));
            }
        }
    }
}