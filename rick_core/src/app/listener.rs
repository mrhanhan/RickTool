use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Error, Formatter, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{RwLock};

#[derive(Debug)]
pub enum EventContextError {
    OnLockError,
    PushNoListener,
    PushLockError,
    PushCallError(String),
}

impl Display for EventContextError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OnLockError => {
                f.write_str("监听方法时，获取锁失败")
            }
            Self::PushNoListener => {
                f.write_str("此事件无监听者")
            }
            Self::PushLockError => {
                f.write_str("图送事件时，获取锁失败")
            }
            Self::PushCallError(ref _msg) => {
                f.write_str(_msg)
            }
        }
    }
}

impl std::error::Error for EventContextError {

}

pub trait Listener {
    /// 处理数据
    fn on_event<'a>(&self, data: &'a dyn Any);
}
/// 类型转换
pub fn convert<'a, T: Any>(data: &'a dyn Any) -> &'a T {
    unsafe {
        &*(data as *const dyn Any as *const T)
    }
}/// 类型转换
pub fn convert_mut<'a, T: Any>(data: &'a dyn Any) -> &'a mut T {
    unsafe {
        &mut *(data as *const dyn Any as *mut T)
    }
}

impl Listener for dyn Fn(&dyn Any) {
    fn on_event<'a>(&self, data: &'a dyn Any) {
        self(data)
    }
}
impl Listener for fn(&dyn Any) {
    fn on_event<'a>(&self, data: &'a dyn Any) {
        self(data)
    }
}

pub trait Event<'a> {
    fn event(&self) -> &'a str;
}

/// 事件系统
pub trait EventContext<'a> {
    /// 监听指定事件
    fn on<F>(&self, event: &'a str, listener: F) -> Result<(), EventContextError>
        where F: Listener + 'static;
    /// Event 对象转换为On
    fn event_on<E: Event<'a>, F>(&self, event: E, listener: F) -> Result<(), EventContextError>
        where F: Listener + 'static {
        self.on(event.event(), listener)
    }
    /// into 进行绑定
    fn into_on<E: Into<&'a str>, F>(&self, event: E, listener: F) -> Result<(), EventContextError>
        where F: Listener + 'static {
        self.on(event.into(), listener)
    }

    /// 推送事件
    fn push(&self, event: &str, data: &dyn Any)-> Result<(), EventContextError>;
    /// Event 对象转换为On
    fn event_push<E: Event<'a>>(&self, event: E, data: &dyn Any) -> Result<(), EventContextError> {
        self.push(event.event(), data)
    }
    /// into 进行绑定
    fn into_push<E: Into<&'a str>>(&self, event: E, data: &dyn Any) -> Result<(), EventContextError> {
        self.push(event.into(), data)
    }

}

/// 简单的事件上下文
pub struct SimpleEventContext<'a> {
    /// 事件MAP
    event_map: RwLock<HashMap<&'a str, Vec<Box<dyn Listener>>>>
}
impl<'a> SimpleEventContext<'a> {

    /// 创建一个新的事件上下文
    pub fn new() -> Self {
        Self {
            event_map: RwLock::new(HashMap::new())
        }
    }
}

impl<'a> Display for SimpleEventContext<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.event_map.read() {
            Ok(_map) => {
                for (_key, _value) in _map.iter() {
                    f.write_str(&format!("Event: \"{}\" Listener Count: [{}]", _key, _value.len())).unwrap();
                }
                Ok(())
            }
            Err(_) => {
                Err(Error)
            }
        }
    }
}

impl<'a> EventContext<'a> for SimpleEventContext<'a> {
    fn on<F>(&self, event: &'a str, listener: F) -> Result<(), EventContextError>
        where F: Listener  + 'static {
        match self.event_map.write() {
            Ok(mut map) => {
                if map.contains_key(event) {
                    let vec = map.get_mut(event).unwrap();
                    vec.push(Box::new(listener));
                } else {
                    map.insert(event, vec![Box::new(listener)]);
                }
                Ok(())
            }
            Err(_) => {
                Err(EventContextError::OnLockError)
            }
        }
    }

    fn push(&self, event: &str, data: &dyn Any) -> Result<(), EventContextError>{
         match self.event_map.read() {
             Ok(_map) => {
                 if let Some(_vec) = _map.get(event) {
                     let result = catch_unwind(AssertUnwindSafe(|| {
                         for _fn in _vec {
                             _fn.on_event(&(*data));
                         }
                     }));
                     if let Err(err) = result {
                         eprintln!("A listener for event {} panicked: {:?}", event, err);
                         return Err(EventContextError::PushCallError(format!("调用失败:{} 错误:{:?}", event, err)));
                     }
                     Ok(())
                 } else {
                     Err(EventContextError::PushNoListener)
                 }
             }
             Err(_) => {
                 Err(EventContextError::PushLockError)
             }
         }
    }
}

#[cfg(test)]
mod tests {
    use std::any::{Any};
    use std::panic::catch_unwind;
    use crate::app::listener::{convert, convert_mut, EventContext, SimpleEventContext};
    struct M {
        pub a: u8,
        pub b: u8
    }

    impl M {

        fn hello(&self) {
            println!("M hello {}:{}", self.a, self.b);
        }
    }
    struct A {
        pub a: [u8;2]
    }
    #[test]
    fn test() {
        let context = SimpleEventContext::new();
        let hello: fn(&dyn Any) = |_data: &dyn Any| {
            let display = convert::<M>(_data);
            display.hello();
            convert_mut::<A>(_data).a[0] = 0;
            convert_mut::<A>(_data).a[1] = 90;
            println!("{:?}", convert::<A>(_data).a);
            let m = 1/ convert::<A>(_data).a[0];
        };
        context.on("hello world", hello).unwrap();
        let s = A{a: [10, 20]};
        println!("origin s: {:p}", &s);
        context.push("hello world", &s);
        context.push("hello world", &s);
        // context.push("hello world", &mut A{a: [10, 20]}).unwrap();
        println!("Context {}", context);
    }

    #[test]
    fn test_read_write() {

        let mut a = [10, 10];
        let b = [20, 20];
        // 修改

        let a1 = unsafe {&a as *const [i32;2] as usize as *mut [u32;2]};
        unsafe {(*a1)[0] = 20;}
        let b1 = unsafe {&b as *const [i32;2] as usize as *mut [u32;2]};
        unsafe {(*b1)[0] = 39;}

        let c = a[0]+ b[0];
    }
}
