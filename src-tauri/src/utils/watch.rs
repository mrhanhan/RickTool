use std::{fmt::Display, time::{SystemTime, UNIX_EPOCH, Duration}};




/// 时间监视器
/// 'a 是任务名称的变量生命周期
/// 
#[allow(unused)]
pub(crate) struct TimeWatch<T: Display + Sized> {
    /// 任务名称
    task_name: T,
    start: Option<std::time::SystemTime>,
    last: Option<std::time::SystemTime>,
} 

/// 时间单位
#[allow(unused)]
pub(crate) enum TimeUnit {
    MILLIS,
    NANOS,
    SECONDS,
}

#[allow(unused)]
impl<T: Display + Sized> TimeWatch<T> {

    /// 开始检测
    pub(crate) fn watch(task_name: T) -> Self {
        let time_now = std::time::SystemTime::now();
        TimeWatch {
            task_name: task_name,
            start: Some(time_now.clone()),
            last: Some(time_now)
        }
    }

    /// 打印任务信息
    pub(crate) fn print(&mut self, unit: TimeUnit) {
        let time_now = std::time::SystemTime::now();
        let now_duration = time_now.duration_since(UNIX_EPOCH).unwrap();
        let last_duration = self.last.unwrap().duration_since(UNIX_EPOCH).unwrap();
        let start_duration = self.start.unwrap().duration_since(UNIX_EPOCH).unwrap();
        let clac = |time1: &Duration, time2: &Duration| -> u128 {
            match &unit {
                TimeUnit::MILLIS => time2.as_millis() - time1.as_millis(),
                TimeUnit::NANOS => time2.as_nanos() - time1.as_nanos(),
                TimeUnit::SECONDS => (time2.as_secs() - time1.as_secs()) as u128,
            }
        };
        println!("{} 总差:{} 单次差距:{}", self.task_name, clac(&start_duration, &now_duration), clac(&last_duration, &now_duration));
        self.last = Some(SystemTime::now());
    }
    
    /// 打印任务信息
    pub(crate) fn print_step(&mut self, step: &str, unit: TimeUnit) {
        let time_now = std::time::SystemTime::now();
        let now_duration = time_now.duration_since(UNIX_EPOCH).unwrap();
        let last_duration = self.last.unwrap().duration_since(UNIX_EPOCH).unwrap();
        let start_duration = self.start.unwrap().duration_since(UNIX_EPOCH).unwrap();
        let clac = |time1: &Duration, time2: &Duration| -> u128 {
            match &unit {
                TimeUnit::MILLIS => time2.as_millis() - time1.as_millis(),
                TimeUnit::NANOS => time2.as_nanos() - time1.as_nanos(),
                TimeUnit::SECONDS => (time2.as_secs() - time1.as_secs()) as u128,
            }
        };
        println!("{} - [{}] 总差:{} 单次差距:{}", self.task_name, step, clac(&start_duration, &now_duration), clac(&last_duration, &now_duration));
        self.last = Some(SystemTime::now());
    }
    
}