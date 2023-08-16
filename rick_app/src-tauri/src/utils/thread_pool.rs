use std::{
    sync::{RwLock, Arc, atomic::{AtomicUsize, Ordering}},
    thread,
    time::Duration,
};

use super::{thread_signal::ThreadSignal};

pub(crate) type Task = Box<dyn FnOnce()>;

/// 任务队列
pub(crate) struct TaskQueue(RwLock<Vec<Task>>, ThreadSignal);

#[allow(unused)]
impl<'a> TaskQueue {
    /// 创建任务队列
    pub(crate) fn new(signal: ThreadSignal) -> Self {
        TaskQueue(RwLock::new(Vec::new()), signal)
    }

    /// 推送任务
    /// ```
    /// TaskQueue::push(Box::new(||{
    ///
    /// }))
    /// ```
    pub(crate) fn push(&self, task: Task) {
        self.0.write().unwrap().push(task);
        // 告知
        self.1.notify_all();
    }

    pub(crate) fn clear(&self) {
        self.0.write().unwrap().clear();
    }

    /// 获取任务数据
    /// timeout 间隔指定时间
    pub(crate) fn pop(&self, timeout: Duration) -> Option<Task> {
        // 读取任务
        let mut task_count = 0;
        {
            let task = self.0.read().unwrap();
            task_count = task.len();
            // 释放锁
        }
        if task_count == 0 {
            // 睡眠
            self.1.wait_timeout(timeout);
        }
        // 判断是否有值
        if task_count > 0 {
            let mut queue = self.0.write().unwrap();
            let task = queue.remove(0);
            return Some(task);
        }
        None
    }
}

unsafe impl Send for TaskQueue {}
unsafe impl Sync for TaskQueue {}

#[derive(Debug)]
pub enum SubmitError {
    STATUS(&'static str),
}

/// 线程池
pub struct ThreadPool {
    /// 核心线程数量
    core_size: usize,
    /// 当前线程数
    current_size: Arc<AtomicUsize>,
    /// 最大线程数
    max_size: usize,
    /// 最大空闲时间
    max_time: Duration,
    /// 任务队列
    queue: Arc<TaskQueue>,
    /// 线程池状态
    status: Arc::<RwLock<bool>>,
}

#[allow(unused)]
impl ThreadPool {
    /// 创建一个线程池
    /// ```
    /// use std::time::Duration;
    /// /// core_size: 线程池核心线程数量
    /// /// max_size: 线程池最大可用线程数量
    /// /// max_time: 临时线程存活最大时间
    /// use rick_core::utils::ThreadPool;
    /// let pool = ThreadPool::new(10, 20, Duration::from_micros(10));
    /// pool.submit(Box::new(|| {
    /// }));
    /// ```
    pub fn new(core_size: usize, max_size: usize, max_time: Duration) -> Self {
        Self {
            core_size,
            current_size: Arc::new(AtomicUsize::new(0)),
            max_size,
            max_time,
            queue: Arc::new(TaskQueue::new(ThreadSignal::new())),
            status: Arc::new(RwLock::new(true)),
        }
    }

    /// 停止线程池
    pub(crate) fn shutdown(&mut self) {
        let mut status = self.status.write().unwrap();
        *status = false;
        self.queue.clear();
    }

    /// 提交任务
    /// ```
    /// 提交任务
    /// ```
    pub fn submit(&self, task: Task) -> Result<(), SubmitError> {
        {
            if !*self.status.read().unwrap() {
                return Result::Err(SubmitError::STATUS("线程池状态错误，可能线程池已停止"));
            }
        }
        {
            self.queue.push(task);
        }
        // 判断是否到达核心线程数量
        let current_size: usize = self.current_size.load(Ordering::Acquire);
        if current_size < self.core_size {
            let queue = self.queue.clone();
            let status = self.status.clone();
            let count = self.current_size.clone();
            // 创建核心线程数
            thread::spawn(move || {
                while *status.read().unwrap() {
                    if let Some(t) = queue.pop(Duration::from_millis(500)) {
                        t();
                    } else {}
                }
                count.fetch_sub(1, Ordering::Acquire);
            });
            self.current_size.fetch_add(1, Ordering::Acquire);
        } else if current_size < self.max_size {
            let queue = self.queue.clone();
            let status = self.status.clone();
            let time = self.max_time.clone();
            let count = self.current_size.clone();
            // 创建临时线程
            thread::spawn(move || {
                while *status.read().unwrap() {
                    if let Some(t) = queue.pop(time) {
                        t();
                    } else {
                        break;
                    }
                }
                count.fetch_sub(1, Ordering::Acquire);
            });
            self.current_size.fetch_add(1, Ordering::Acquire);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use crate::utils::ThreadPool;

    #[test]
    fn test() {
        let pool = ThreadPool::new(10, 20, Duration::from_millis(100));
        let model = 1;
        pool.submit(Box::new(move || {
            println!("{}", model);
        })).unwrap();
        sleep(Duration::from_millis(1000));
    }
}