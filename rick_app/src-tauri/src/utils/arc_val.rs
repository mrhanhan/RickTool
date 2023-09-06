use crate::utils::convert_mut;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

pub struct ArcVal<T: Sized> {
    value: Arc<Box<T>>,
    lock: Arc<Mutex<bool>>,
}

impl<T: Sized + 'static> ArcVal<T> {
    /// 创建新的值
    pub fn new(data: T) -> Self {
        Self {
            value: Arc::new(Box::new(data)),
            lock: Arc::new(Mutex::new(false)),
        }
    }

    pub fn using<F: Fn(&mut T)>(&self, func: F) -> &ArcVal<T> {
        {
            let _lock = self.lock.lock().unwrap();
            let data = self.value.deref().deref();
            let mut_value = convert_mut::<T>(data);
            func(mut_value);
        }
        self
    }
}

impl<T: Sized + 'static> Deref for ArcVal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let _value: &Self::Target = self.value.deref().deref();
        _value
    }
}

impl<T> Clone for ArcVal<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            lock: self.lock.clone(),
        }
    }
}
