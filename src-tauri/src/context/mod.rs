use std::{sync::{Arc}, any::Any, time::Duration};
use crate::utils::{thread_pool::ThreadPool, thread_signal::ThreadSignal};

use self::{events::{ Publisher, SimplePublisher}, api_manager::{ApiManager, DefaultApiManager}};

pub(crate) mod events;
pub mod api_manager;
pub mod api_handler;

/**
 * 应用程序
 */
#[allow(unused)]
pub(crate) trait Application: Publisher + ApiManager{

    /// 初始化
    fn init(&self);
    
    /// 停止
    fn stop(&self);
    
    /// 启动
    fn start(&self);

    /// 异步推送
    fn publish_async(&self, event: events::Event, data: Box<dyn Any + Send + Sync + 'static>);
    
}


#[allow(unused)]
pub(crate) struct DefaultApplication<P: Publisher>{
    /// 事件发布器
    publister: Arc<P>,
    executor: ThreadPool,
    api_manager: DefaultApiManager,
    signal: ThreadSignal
}

pub(crate) const INIT_BEFORE: &str = "INIT_BEFORE";
pub(crate) const INIT: &str = "INIT";
pub(crate) const INIT_AFTER: &str = "INIT_AFTER";
pub(crate) const START_BEFORE: &str = "START_BEFORE";
pub(crate) const START: &str = "START";
pub(crate) const START_AFTER: &str = "START_AFTER";
pub(crate) const STOP_BEFORE: &str = "STOP_BEFORE";
pub(crate) const STOP: &str = "STOP";
pub(crate) const STOP_AFTER: &str = "STOP_AFTER";

impl Publisher for DefaultApplication<SimplePublisher> {
    fn add_subscriber(&self, event: events::Event, subscriber: events::Subscriber) {
        self.publister.add_subscriber(event, subscriber);
    }

    fn clear_subscriber(&self, event: events::Event) {
        self.publister.clear_subscriber(event);
    }

    fn publish(&self, event: events::Event, data: Box<dyn Any>) {
        self.publister.publish(event, data);
    }
}

impl ApiManager for DefaultApplication<SimplePublisher> {
    fn register_api(&self, operate: String, handler: Box<api_manager::ApiHandler>) -> Result<(), api_manager::ErrorEnum> {
        self.api_manager.register_api(operate, handler)
    }

    fn set_default_api(&self, handler: Box<api_manager::ApiHandler>) {
        self.api_manager.set_default_api(handler)
    }

    fn call_api(&self, context: &mut api_manager::CallContext) -> Result<String, String> {
        self.api_manager.call_api(context)
    }
}

impl Application for DefaultApplication<SimplePublisher> {
    fn init(&self) {
        self.publish(INIT_BEFORE, Box::new(Option::<i32>::None));
        self.publish(INIT, Box::new(Option::<i32>::None));
        self.publish(INIT_AFTER, Box::new(Option::<i32>::None));
    }
    

    fn stop(&self) {
        println!("程序关闭");
        self.signal.notify_one();
    }

    fn start(&self) {
        self.publish(START_BEFORE, Box::new(Option::<i32>::None));
        self.publish(START, Box::new(Option::<i32>::None));
        self.publish_async(START_AFTER, Box::new(Option::<i32>::None));
        self.signal.wait();
        self.publish(STOP_BEFORE, Box::new(Option::<i32>::None));
        self.publish(STOP, Box::new(Option::<i32>::None));
        self.publish(STOP_AFTER, Box::new(Option::<i32>::None));
    }

    fn publish_async(&self, event: events::Event, data: Box<dyn Any + Send + Sync + 'static>) {
        let publister = self.publister.clone();
        self.executor.submit(Box::new( move || {
            publister.publish(event, data);
        })).unwrap();
    }
}

impl DefaultApplication<SimplePublisher> {
    
    fn new() -> Self {
        DefaultApplication::<SimplePublisher>{
            publister: Arc::new(SimplePublisher::new()),
            signal: ThreadSignal::new(),
            api_manager: DefaultApiManager::new(),
            executor: ThreadPool::new(10, 1024, Duration::from_secs(30)),
        }
    }
}

lazy_static! {
    static ref GLOBAL_APPLICATION: Arc<DefaultApplication<SimplePublisher>> = Arc::new(DefaultApplication::new());
}

pub(crate) fn use_application() -> Arc<impl Application> {
    GLOBAL_APPLICATION.clone()
}
/// 获取共享Application 的指针
#[allow(unused)]
pub(crate) fn use_application_ptr() -> *const impl Application {
   Arc::into_raw(GLOBAL_APPLICATION.clone())
}
