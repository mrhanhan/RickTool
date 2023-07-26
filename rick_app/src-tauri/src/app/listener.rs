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


// pub trait Listener<D> {
//     /// 处理数据
//     fn on_event(&self, data: D);
// }
// pub type BoxListener<D> = Box<dyn Listener<D>>;

pub trait Event {
    fn event(&self) -> String;
}

impl Event for &str {
    fn event(&self) -> String {
        String::from(*self)
    }
}

//
// /// 事件系统
// pub trait EventContext<D> {
//     /// 监听指定事件
//     fn on(&self, event: String, listener: Box<dyn Listener<D>>) -> Result<(), EventContextError>;
//     /// Event 对象转换为On
//     fn event_on<E: Event>(&self, event: E, listener: Box<dyn Listener<D>>) -> Result<(), EventContextError> {
//         self.on(event.event(), listener)
//     }
//     /// into 进行绑定
//     fn on_into<E: Into<String>, F>(&self, event: E, listener: Box<dyn Listener<D>>) -> Result<(), EventContextError> {
//         self.on(event.into(), listener)
//     }
//     /// 推送事件
//     fn push(&self, event: String, data: D) -> Result<(), EventContextError>;
//     /// Event 对象转换为On
//     fn push_event<E: Event>(&self, event: E, data: D) -> Result<(), EventContextError> {
//         self.push(event.event(), data)
//     }
//     /// into 进行绑定
//     fn push_into<E: Into<String>>(&self, event: E, data: D) -> Result<(), EventContextError> {
//         self.push(event.into(), data)
//     }
//
// }
//
// /// 简单的事件上下文
// pub struct SimpleEventContext<D> {
//     /// 事件MAP
//     event_map: Arc<RwLock<HashMap<String, Vec<Box<dyn Listener<D>>>>>>,
// }
//
// impl<D> SimpleEventContext<D> {
//     /// 创建一个新的事件上下文
//     pub fn new() -> Self {
//         Self {
//             event_map: Arc::new(RwLock::new(HashMap::new()))
//         }
//     }
// }
//
// impl<D> Display for SimpleEventContext<D> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self.event_map.read() {
//             Ok(_map) => {
//                 for (_key, _value) in _map.iter() {
//                     f.write_str(&format!("Event: \"{}\" Listener Count: [{}]", _key, _value.len())).unwrap();
//                 }
//                 Ok(())
//             }
//             Err(_) => {
//                 Err(Error)
//             }
//         }
//     }
// }
//
// impl<D> EventContext<D> for SimpleEventContext<D> {
//     fn on(&self, event: String, listener: BoxListener<D>) -> Result<(), EventContextError> {
//         match self.event_map.write() {
//             Ok(mut map) => {
//                 if map.contains_key(&event) {
//                     let vec = map.get_mut(&event).unwrap();
//                     vec.push(listener);
//                 } else {
//                     map.insert(event, vec![listener]);
//                 }
//                 Ok(())
//             }
//             Err(_) => {
//                 Err(EventContextError::OnLockError)
//             }
//         }
//     }
//
//     fn push(&self, event: String, data: D) -> Result<(), EventContextError> {
//         match self.event_map.read() {
//             Ok(_map) => {
//                 if let Some(_vec) = _map.get(&event) {
//                     let result = catch_unwind(AssertUnwindSafe(|| {
//                         for _fn in _vec {
//                             _fn.on_event((*data));
//                         }
//                     }));
//                     if let Err(err) = result {
//                         eprintln!("A listener for event {} panicked: {:?}", &event, err);
//                         return Err(EventContextError::PushCallError(format!("调用失败:{} 错误:{:?}", event, err)));
//                     }
//                     Ok(())
//                 } else {
//                     Err(EventContextError::PushNoListener)
//                 }
//             }
//             Err(_) => {
//                 Err(EventContextError::PushLockError)
//             }
//         }
//     }
// }
//
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
