use std::sync::Mutex;
use std::thread::{sleep, Thread};
use std::time::{Duration, SystemTime};
use lazy_static::lazy_static;

pub struct Snow {
    /// 开始ID
    start_time: u64,
    /// Worker ID
    worker_id: u32,
    /// 中心ID
    center_id: u32,
    /// 序列
    sequence: Mutex<u32>,
    /// 上次生成的时间
    last_time: Mutex<u64>,
}

pub fn system_now() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() * 1000u64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as u64;
    ms
}

impl Snow {
    /// 创建雪花算法ID
    /// @param worker_id 工作区ID
    /// @param center_id 数据中心ID
    pub fn new(worker_id: u32, center_id: u32) -> Self {
        let start_time = system_now();
        Self {
            worker_id,
            center_id,
            start_time,
            sequence: Mutex::new(0),
            last_time: Mutex::new(start_time),
        }
    }

    pub fn gen(&self) -> u64 {
        let time: u64 = system_now();
        let last_time: u64 = { *(self.last_time.lock().unwrap()) };
        let mut seq = { *(self.sequence.lock().unwrap()) };
        // 时间一致
        if time == last_time {
            // 判断序号是否溢出，如果是，则
            if seq + 1 >= 4096 {
                // 等待
                sleep(Duration::from_millis(1));
                return self.gen();
            } else {
                seq = seq + 1;
            }
        } else {
            // 时间改变了,从之序号
            seq = 0;
        }
        { *(self.sequence.lock().unwrap()) = seq };
        let mut id = 0_u64;
        id |= time;
        id <<= 5;
        id |= self.worker_id as u64;
        id <<= 5;
        id |= self.center_id as u64;
        id <<= 12;
        id |= seq as u64;
        id
    }
}

lazy_static! {
    static ref SNOW: Snow = Snow::new(100, 100);
}

pub fn snow_id() -> u64 {
    SNOW.gen()
}
pub fn snow_hex() -> String {
    String::from(&(format!("{:#x}", SNOW.gen()).as_str()[2..]))
}

#[cfg(test)]
mod tests {
    use crate::utils::snow::{Snow, snow_hex};

    #[test]
    fn test_gen() {
        let id: Snow = Snow::new(0, 0);

        println!("{} {}", id.gen(), snow_hex());
        println!("{} {}", id.gen(), snow_hex());
        println!("{} {}", id.gen(), snow_hex());
        println!("{} {}", id.gen(), snow_hex());
    }
}