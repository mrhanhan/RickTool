use std::{sync::{Condvar, Mutex, Arc}, time::Duration};


pub(crate) struct ThreadSignal(Condvar, Mutex<bool>);

#[allow(unused)]
impl ThreadSignal {

    pub(crate) fn new_arc() -> Arc<ThreadSignal> {
        Arc::new(ThreadSignal::new())
    }

    pub(crate) fn new() -> Self {
        ThreadSignal(Condvar::new(), Mutex::new(false))
    }

    /// 等待
    pub(crate) fn wait(&self) {
        let mut flag = self.1.lock().unwrap();
        // 当前版本
        if *self.0.wait(flag).unwrap() {
            *self.1.lock().unwrap() = false;
        }
    }
    /// 等待
    pub(crate) fn wait_timeout(&self, timeout: Duration) {
        let mut flag = self.1.lock().unwrap();
        match self.0.wait_timeout(flag, timeout) {
            Ok(mut f) => {
                if *f.0 {
                    *f.0 = false;
                }
            },
            Err(_) => {
                let mut f = self.1.lock().unwrap();
                if *f {
                    *f = false;
                }
                return;
            }
        }
    }

    /// 唤醒一个
    pub(crate) fn notify_one(&self) {
        *self.1.lock().unwrap() = true;
        self.0.notify_one();
    }
    /// 唤醒一个
    pub(crate) fn notify_all(&self) {
        *self.1.lock().unwrap() = true;
        self.0.notify_all();
    }

}