mod global_val;
mod thread_pool;
mod thread_signal;
mod arc_val;

use std::any::Any;
pub use thread_pool::*;
pub use thread_signal::*;
pub use global_val::*;
pub use arc_val::*;


/// 类型转换
pub fn convert<'a, T: Any>(data: &'a dyn Any) -> Box<T> {
    unsafe {
        Box::<T>::from_raw(data as *const dyn Any as *mut T)
    }
}

/// 类型转换
pub fn convert_ref<'a, T: Any>(data: &'a dyn Any) -> &'a T {
    unsafe {
        &*(data as *const dyn Any as *const T)
    }
}

/// 类型转换
pub fn convert_mut<'a, T: Any>(data: &'a dyn Any) -> &'a mut T {
    unsafe {
        &mut *(data as *const dyn Any as *mut T)
    }
}
