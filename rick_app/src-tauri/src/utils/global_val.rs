#[derive(Debug)]
pub struct GlobalVal<T> {
    /// 指针
    ptr: usize,
    data: Option<T>,
}

impl<T> Clone for GlobalVal<T> {
    fn clone(&self) -> Self {
        GlobalVal {
            ptr: self.ptr,
            data: None,
        }
    }
}

impl<T> GlobalVal<T> {
    /// 创建全局对象
    pub const fn new() -> Self {
        GlobalVal { ptr: 0, data: None }
    }
    /// 设置全局数据
    pub fn set(&mut self, data: T) {
        self.data = Some(data);
        self.ptr = (self.data.as_ref().unwrap() as *const T) as usize;
    }

    pub fn get_ref(&self) -> Option<&T> {
        if self.ptr != 0 {
            return unsafe {
                let data = self.ptr as *const T;
                Some(&(*data))
            };
        }
        None
    }
    pub fn get_mut_ref(&self) -> Option<&mut T> {
        if self.ptr != 0 {
            return unsafe {
                let data = self.ptr as *mut T;
                Some(&mut (*data))
            };
        }
        None
    }
}

/// 定义全局变量
/// ```
/// global_val!(变量名称, 类型)
/// ```
/// 使用变量
/// ```
/// let val = global_val!(变量名称);
/// ```
#[macro_export]
macro_rules! global_val {
    ($name:ident, $type:ty) => {
        static mut $name: GlobalVal<$type> = GlobalVal::new();
    };
    ($name:ident) => {
        unsafe { $name.clone() }
    };
}
#[macro_export]
macro_rules! global_val_set {
    ($name:ident, $value:expr) => {
        unsafe { $name.set($value) };
    };
}

pub struct SharedVal {
    /// 值
    ptr: usize,
}

#[cfg(test)]
mod tests {
    use crate::utils::global_val::GlobalVal;
    use std::mem::size_of;
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;

    #[derive(Debug)]
    struct Global {
        count: i32,
    }

    static mut GLOBAL: GlobalVal<Global> = GlobalVal::new();

    #[test]
    fn test_ptr() {
        unsafe { GLOBAL.set(Global { count: 0 }) }
        let val = unsafe { GLOBAL.clone() };
        let data1 = val.get_mut_ref().unwrap();
        let data2 = val.get_mut_ref().unwrap();
        data1.count = data1.count + 1;
        data2.count = data2.count + 1;
        data2.count = data2.count + 1;
        data2.count = data2.count + 1;
        data2.count = data2.count + 1;
        println!("Global: {:?}", &val);
        println!("Usize: {:?}", size_of::<usize>());
        println!("Global :{:p}", &val);
        println!("Global Data :{:p}", &val);
        println!("Global Ptr: 0x{:x}", val.ptr);
        println!("Data1: {:?}", data1);
        println!("Data1 Ptr: {:p}", data1);
        println!("Data2: {:?}", data2);
        println!("Data2 Ptr: {:p}", data2);
    }

    #[test]
    fn test_ptr_thread() {
        unsafe { &mut GLOBAL }.set(Global { count: 0 });
        let val = unsafe { GLOBAL.clone() };
        let val1 = val.clone();
        thread::spawn(move || {
            let mut c = val1.get_mut_ref().unwrap();
            for i in 0..100 {
                c.count = c.count + 1;
                sleep(Duration::from_millis(10))
            }
        });
        thread::spawn(move || {
            let c = val.get_ref().unwrap();
            loop {
                println!("{:?}", c);
                sleep(Duration::from_millis(100))
            }
        });
        sleep(Duration::from_millis(20000));
    }

    global_val!(HELLO, i32);

    #[test]
    fn test_global_macro() {
        global_val_set!(HELLO, 1);
        println!("{:?}", global_val!(HELLO).get_ref());
    }
}
