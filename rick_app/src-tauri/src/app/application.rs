use std::process::exit;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{App, RunEvent, Wry};

use std::time::Duration;
use crate::app::listener::{EventBus};
use crate::app::module::ModuleManager;
use crate::app::service::ServiceRegister;
use crate::define_event;
use crate::global::{RickApp, RickAppHandler};
use crate::utils::{ThreadPool, ThreadSignal};

/// 应用程序事件
define_event!(ApplicationEvent => Started, Stoped);


/// RickTool 的应用程序功能
pub struct Application {
    /// 程序
    _app: Arc<Mutex<Option<RickApp>>>,
    /// 应用程序上下文
    _app_handler: Arc<RwLock<Option<RickAppHandler>>>,
    /// 事件发布器
    _event_context: Arc<EventBus>,
    /// 线程池
    _thread_pool: Arc<ThreadPool>,
    ///
    _status: Arc<(ThreadSignal, AtomicBool)>,
    /// 服务注册器
    _service_register: ServiceRegister,
    /// 模块管理器
    _module_manager: ModuleManager
}


#[allow(unused)]
impl Application {
    /// 创建一个Application
    pub fn new(app: App<Wry>) -> Self {
        Application {
            _app_handler: Arc::new(RwLock::new(Some(app.handle()))),
            _app: Arc::new(Mutex::new(Option::Some(app))),
            _event_context: Arc::new(EventBus::new()),
            _thread_pool: Arc::new(ThreadPool::new(10, 1024, Duration::from_millis(10))),
            _status: Arc::new((ThreadSignal::new(), AtomicBool::new(false))),
            _service_register: ServiceRegister::new(),
            _module_manager: ModuleManager::new()
        }
    }
}

unsafe impl Send for Application {}

#[allow(unused)]
impl Application {
    /// 返回事件上下文
    pub fn event_context(&self) -> Arc<EventBus> {
        self._event_context.clone()
    }
    /// 线程池
    pub fn thread_pool(&self) -> Arc<ThreadPool> {
        self._thread_pool.clone()
    }

    pub fn app_handler(&self) -> RickAppHandler {
        let app = self._app_handler.read().unwrap();
        app.as_ref().unwrap().clone()
    }
    pub fn module_manager(&self) -> ModuleManager {
        self._module_manager.clone()
    }
    pub fn service_register(&self) -> ServiceRegister {
        self._service_register.clone()
    }
    fn set_context(&self, context: RickAppHandler) {
        let mut ctx = self._app_handler.write().unwrap();
        *ctx = Some(context);
    }
    /// 启动
    pub fn start(&mut self) {
        // 启动过
        if self._status.1.load(Ordering::Acquire) {
            return;
        }
        self._status.1.store(true, Ordering::Release);
        // 启动模块注册服务
        let app = self.clone();
        self.thread_pool().submit(Box::new(move || {
            // 初始化
            app._module_manager.on_init(app.clone());
        }));
        let mut mutex_guard = self._app.lock().unwrap();
        let app = mutex_guard.take().unwrap();
        let ctx = self as *const Application as usize;
        app.run(move |_handler, _event| {
            let application = unsafe {&*(ctx as *const Application)};
            let event_context = application.event_context();
            match _event {
                RunEvent::Ready => {
                    event_context.emit_into(ApplicationEvent::Started, application.clone());
                },
                RunEvent::ExitRequested {api, .. } => {
                    println!("ExitRequested");
                    event_context.emit_into(ApplicationEvent::Stoped, application.clone());
                },
                _ => {}
            }
        });
    }

    /// Clone
    pub fn clone(&self) -> Application {
        Application {
            _thread_pool: self.thread_pool(),
            _event_context: self._event_context.clone(),
            _app: self._app.clone(),
            _status: self._status.clone(),
            _app_handler: self._app_handler.clone(),
            _service_register: self._service_register.clone(),
            _module_manager: self._module_manager.clone()
        }
    }
}