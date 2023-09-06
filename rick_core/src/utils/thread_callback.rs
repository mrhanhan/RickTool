use std::cell::RefCell;
thread_local! {
    static CALLBACK: RefCell<Vec<Box<dyn Fn(&ThreadExecuteStatus)>>> = RefCell::new(Vec::new());
}

#[derive(Clone, Debug)]
pub enum ThreadExecuteStatus {
    Panic,
    Ok,
}

pub fn add_callback(_callback: Box<dyn Fn(&ThreadExecuteStatus)>) {
    CALLBACK.with(|d| {
        d.borrow_mut().push(_callback);
    });
}
// 调用 callback
pub fn call_callback(status: ThreadExecuteStatus) {
    CALLBACK.with(|d| {
        let _callback_vec = &*d.borrow();
        for _callback in _callback_vec {
            _callback(&status);
        }
    })
}
