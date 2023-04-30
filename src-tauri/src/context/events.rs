use std::{any::Any, collections::HashMap, sync::RwLock};

// 定义事件类型
pub(crate) type Event = &'static str;


/// 订阅者
pub(crate) type Subscriber = Box<dyn Fn(Event, &dyn Any) + Send + Sync>;

/// 发布者
pub(crate) trait Publisher {
    /// 添加订阅者
    fn add_subscriber(&self, event: Event, subscriber: Subscriber);
    /// 清除订阅者
    fn clear_subscriber(&self, event: Event);
    /// 发布数据
    fn publish(&self, event: Event, data: Box<dyn Any>);
}


pub(crate) struct SimplePublisher {
    /// 订阅者Map
    subscriber_map: RwLock<HashMap<Event, Vec<Subscriber>>>
}

impl SimplePublisher {

    pub(crate) fn new() -> Self {
        SimplePublisher {
            subscriber_map: RwLock::new(HashMap::new())
        }
    }
}

impl Publisher for SimplePublisher{
       /// 添加订阅者
    fn add_subscriber(&self, event: Event, subscriber: Subscriber) {
        match self.subscriber_map.write() {
            Ok(mut _map) => {
                if !_map.contains_key(event) {
                    _map.insert(event, Vec::new());
                }
                _map.get_mut(event).unwrap().push(subscriber);
            },
            Err(_) => {

            }
        }
    }

    fn clear_subscriber(&self, event: Event) {
        match self.subscriber_map.write() {
            Ok(mut _map) => {
                _map.remove(event);
            },
            Err(_) => {

            }
        }
    }

    fn publish(&self, event: Event, data: Box<dyn Any>) {
        match self.subscriber_map.read() {
            Ok(_map) => {        
                if _map.contains_key(event) {
                    if let Some(_subscribers) = _map.get(event) {
                        for item in _subscribers {
                            item(event, &data);
                        }
                    }
                }
            },
            Err(_) => {
                
            }
        }
    }
}