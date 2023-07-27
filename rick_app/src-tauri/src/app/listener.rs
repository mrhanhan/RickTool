use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex};
///
/// 定义事件，自动实现 Into 函数
/// ```
/// use rick_core::define_event;
/// define_event!(AppEvent => Start, End, Stop);
/// // 展开如下
/// enum XXEvent {
///     Start, End, Stop
/// }
/// impl Into<&'static str> for XXEvent {
///     fn into(self) -> &'static str {
///         match self {
///             Self::Start => "AppEvent::Start",
///             Self::End => "AppEvent::End",
///             Self::Stop => "AppEvent::Stop"
///         }
///     }
/// }
/// ```
/// 同时支持同时定义多个
/// ```
/// use rick_core::define_event;
/// define_event!(
///     AppEvent => Start, End, Stop;
///     LoginEvent => Error, Login, Logout
/// );
/// ```
#[macro_export]
macro_rules! define_event {

    ($name:ident => $($item:ident),+) => {
        #[derive(Debug)]
        pub enum $name{
            $(
                $item
            ),*
        }
        impl ToString for $name {
            fn to_string(&self) -> String {
                 match self {
                     $(
                     Self::$item => String::from(concat!(module_path!(), "::", stringify!($name), "::", stringify!($item)))
                     ),*
                }
            }
        }
        impl Into<&'static str> for $name {
            fn into(self) -> &'static str {
                match self {
                     $(
                     Self::$item => concat!(module_path!(), "::", stringify!($name), "::", stringify!($item))
                     ),*
                }
            }
        }
        impl Into<String> for $name {
            fn into(self) -> String {
                String::from(Into::<&'static str>::into(self))
            }
        }
    };
    ($($name:ident => $($exp:ident),+);*) => {
        $(
            define_event!($name => $($exp),*);
        )*
    };
    ($($name:ident => $($exp:ident),+;)*) => {
        $(
            define_event!($name => $($exp),*);
        )*
    };
}

pub trait Event {
    fn event(&self) -> String;
}

impl Event for &str {
    fn event(&self) -> String {
        String::from(*self)
    }
}

type BoxedCallback = Box<dyn Fn(&dyn Any) + Send>;

pub struct EventBus {
    handlers: Arc<Mutex<HashMap<String, Vec<BoxedCallback>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            handlers: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn on<T: 'static, F: Fn(&T) + 'static + Send>(&self, event: String, handler: F) {
        let handlers = self.handlers.clone();
        let mut handlers = handlers.lock().unwrap();
        handlers.entry(event)
            .or_insert_with(Vec::new)
            .push(Box::new(move |data: &dyn Any| {
                if let Some(data) = data.downcast_ref::<T>() {
                    handler(data);
                }
            }));
    }

    pub fn on_event<T: 'static, F: Fn(&T) + 'static + Send, I: Event>(&self, event: I, handler: F) {
        self.on(event.event(), handler);
    }
    pub fn on_into<T: 'static, F: Fn(&T) + 'static + Send, I: Into<String>>(&self, event: I, handler: F) {
        self.on(event.into(), handler);
    }

    pub fn emit<T: 'static + Send>(&self, event: String, data: T) {
        let handlers = self.handlers.clone();
        let handlers = handlers.lock().unwrap();
        if let Some(handlers) = handlers.get(&event) {
            for handler in handlers {
                handler(&data);
            }
        }
    }
    pub fn emit_into<T: 'static + Send, I: Into<String>>(&self, event: I, data: T) {
        self.emit(event.into(), data);
    }
    pub fn emit_event<T: 'static + Send, I: Event>(&self, event: I, data: T) {
        self.emit(event.event(), data);
    }
}
