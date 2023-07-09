// use tauri::{AppHandle, Wry};

use crate::app::listener::{EventContext, SimpleEventContext};

/// 应用程序配置
pub struct ApplicationConfig {

}

/// 应用事件
pub enum ApplicationEvent {
    Start
}
impl Into<&'static str> for ApplicationEvent {
    fn into(self) -> &'static str {
        match self { ApplicationEvent::Start => "_application_event_start" }
    }
}


/// RickTool 的应用程序功能
pub struct Application {
    /// 应用程序上下文
    context: Option<usize>,
    _event_context: SimpleEventContext<'static>
}


#[allow(unused)]
impl Application {
    /// 创建一个Application
    pub fn new() -> Self {
        Application {context: None, _event_context: SimpleEventContext::new()}
    }
}

#[allow(unused)]
impl Application {
    /// 返回事件上下文
    pub fn event_context(&self) -> &impl EventContext<'static> {
        &self._event_context
    }
    /// 启动
    pub fn start(&self) {
        self.event_context().into_push(ApplicationEvent::Start, &"");
    }
}