
use std::{sync::{RwLock, Arc}, collections::HashMap, str::FromStr};


pub(crate) type ApiHandler = dyn Fn(&mut CallContext) -> Result<String, String> + Sync + Send; 
#[allow(unused)]
#[derive(Debug)]
pub(crate) enum ErrorEnum {
    Exists,
    Other
}

/// 数据调用上下文
#[allow(unused)]
pub(crate) struct CallContext<'a> {
    pub(crate) window: &'a tauri::Window,
    pub(crate) handler: &'a tauri::AppHandle,
    pub(crate) data:&'a String,
    pub(crate) operate: &'a String
}

impl <'a> CallContext<'a> {
    
    #[allow(unused)]
    pub(crate) fn new(window: &'a tauri::Window, handler: &'a tauri::AppHandle, data: &'a String, operate: &'a String) -> Self { Self { window, handler, data, operate } }
}

/// API 管理器
#[allow(unused)]

pub(crate) trait ApiManager {
    
    /// 注册Api
    /// ```
    /// operate API的操作或者路由
    /// handler 处理方法
    /// ```
    fn register_api(&self, operate: String, handler: Box<ApiHandler>) -> Result<(), ErrorEnum>;
    
    /// 设置默认的API， 在调用的时候，如果API 为空则设置默认API
    fn set_default_api(&self, handler: Box<ApiHandler>);

    /// 调用注册的API 方法
    fn call_api(&self, context: &mut CallContext) -> Result<String, String>;
}

/// 默认API 管理器
#[allow(unused)]
pub(crate) struct DefaultApiManager {
    map: Arc<RwLock<HashMap<String, Arc<Box<ApiHandler>>>>>,
    default: Arc<RwLock<Box<ApiHandler>>>
}

#[allow(unused)]
impl DefaultApiManager {

    /// 创建一个Api 管理器
    pub(crate) fn new() -> Self {
        DefaultApiManager { map: Arc::new(RwLock::new(HashMap::new())), default: Arc::new(RwLock::new(Box::new(|data|{ Err(String::from_str("404").unwrap()) })))}
    }   
}

/// API 管理器
#[allow(unused)]
impl ApiManager for DefaultApiManager {
    fn register_api(&self, operate: String, handler: Box<ApiHandler>) -> Result<(), ErrorEnum> {
        match self.map.write() {
            Ok(mut _map) => {
                if _map.contains_key(&operate) {
                    return Err(ErrorEnum::Exists);
                }
                _map.insert(operate, Arc::new(handler));
                Ok(())
            },
            Err(_) => {
                Err(ErrorEnum::Other)
            }
        }
    }

    fn set_default_api(&self, handler: Box<ApiHandler>) {
        match self.default.write() {
            Ok(mut _default) => {
                *_default = handler;
            },
            Err(_) => {}
        }
    }

    fn call_api(&self, ctx: &mut CallContext) -> Result<String, String> {
        let mut handler_opt: Option<Arc<Box<ApiHandler>>> = None;
        {
            if let Ok(_map) = self.map.read() {
                if let Some(_handler) = _map.get(ctx.operate) {
                    handler_opt = Some(_handler.clone());
                }
            }
        }
        // 判断是否使用默认的
        if let Some(_handler) = handler_opt {
            return _handler(ctx)
        } else {
            if let Ok(_default) = self.default.read() {
                return _default(ctx);
            }
        }
        Err(String::from_str("404").unwrap())
    }
}