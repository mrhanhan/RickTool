use serde::Serialize;
use tauri::{Manager};
use crate::app::application::Application;
use crate::app::module::Module;
use crate::global::RickWindow;

/// 日志级别
#[derive(Debug, Serialize)]
pub enum LogLevel {
    DEBUG,
    INFO,
}
#[derive(Serialize)]
pub struct AppLog<'a, S: Serialize> {
    /// 日志级别
    level: LogLevel,
    module: Option<&'a str>,
    /// 类型
    mtype: &'a str,
    /// 日志消息
    message: &'a str,
    /// payload
    payload: Option<S>
}

impl<'a, S: Serialize> AppLog<'a, S> {}

const APP_LOG_EVENT: &str = "SYSTEM::APP_LOG";

impl<'a, S: Serialize> AppLog<'a, S>{

    /// 发送所有日志
    pub fn send_all(&self, _app: &Application) {
        // 提交日志
        _app.app_handler().emit_all(APP_LOG_EVENT, self).unwrap();
    }

    /// 发送所有日志
    pub fn send_window(&self, _window: &RickWindow) {
        // 提交日志
        _window.emit(APP_LOG_EVENT, self).unwrap();
    }

}

impl<'a, S: Serialize> AppLog<'a, S>{
    /// 床架新的日志
    pub fn new(level: LogLevel, module: Option<&dyn Module>, mtype: &'a str, message: &'a str, payload: Option<S>) -> Self {
        Self {
            level,
            module: module.map(|_m| {_m.meta().name()}),
            mtype,
            message,
            payload
        }
    }
    /// 床架新的日志
    pub fn debug(module: Option<&dyn Module>, mtype: &'a str, message: &'a str, payload: Option<S>) -> Self {
        Self {
            level: LogLevel::DEBUG,
            module: module.map(|_m| {_m.meta().name()}),
            mtype,
            message,
            payload
        }
    }
    /// 床架新的日志
    pub fn info(module: Option<&dyn Module>, mtype: &'a str, message: &'a str, payload: Option<S>) -> Self {
        Self {
            level: LogLevel::INFO,
            module: module.map(|_m| {_m.meta().name()}),
            mtype,
            message,
            payload
        }
    }
}

impl<'a> AppLog<'a, i32> {

    /// 床架新的日志
    pub fn simple(level: LogLevel, message: &'a str) -> Self {
        Self {
            level,
            module: None,
            mtype: "",
            message,
            payload: Option::<i32>::None
        }
    }

    /// 床架新的日志
    pub fn module_simple(level: LogLevel, module: &dyn Module, message: &'a str) -> Self {
        Self {
            level,
            module: Some(module.meta().name()),
            mtype: "",
            message,
            payload: Option::<i32>::None
        }
    }

}
