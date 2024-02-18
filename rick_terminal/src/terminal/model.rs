use std::sync::{Arc, Mutex};
use std::time::Duration;
use log::info;
use rick_core::error::AppError;
use crate::terminal::Size;

pub trait StartupCycle{
    /// 启动之前
    /// @param info 启动程序信息
    /// @return true 继续启动, false 取消启动
    fn on_start(&self, info: &mut StartupInfo);
    /// 程序已经启动
    /// @param info 程序启动信息
    /// @param holder 程序运行信息
    fn on_started(&self, info: &StartupInfo, holder: &dyn ProcessHolder);
    /// 程序运行结束
    /// @param info 程序启动信息
    /// @param holder 程序运行信息
    fn on_end(&self, info: &StartupInfo, holder: &dyn ProcessHolder);
    /// 数据读取操作 可以对读取的数据进行修改
    /// @param info 程序启动信息
    /// @param holder 程序运行信息
    /// @param data 传递的数据信息
    fn on_read(&self, info: &StartupInfo, holder: &dyn ProcessHolder, data: &mut Vec<u8>);
    /// 数据写入 可以对数据进行修改
    /// @param info 程序启动信息
    /// @param holder 程序运行信息
    /// @param data 传递的数据信息
    fn on_write(&self, info: &StartupInfo, holder: &dyn ProcessHolder, data: &mut Vec<u8>);
}

/// 程序信息
pub trait ProcessHolder {
    /// 程序运行信息
    /// @return 当前软件中程序运行ID
    fn id(&self) -> usize;
    /// 程序ID
    /// @return 返回操作系统中进程ID
    fn process_id(&self) -> usize;
    /// 线程ID
    /// @return 返回操作系统中线程ID
    fn thread_id(&self) -> usize;
    /// 启动时间 Unix 时间戳
    /// @return 返回启动时间戳
    fn start_time(&self) -> u64;
    /// 写入数据
    /// @param data 写入的数据
    /// @return 返回写入数据长度
    fn write(&self, data: &mut Vec<u8>) -> Result<usize, AppError>;
    /// 读取数据
    /// @param data 写入的数据
    /// @return 返回读取数据长度
    fn read(&self, data: &mut Vec<u8>) -> Result<usize, AppError>;
    /// 重新设置虚拟终端大小
    /// @param size 大小
    /// @return 返回是否Resize 成功
    fn resize(&self, size: Size) -> Result<(), AppError>;

    /// kill 停止
    /// @return 返回程序停止代码
    fn kill(&self)-> Result<usize, AppError>;
    /// 等待进程结束
    /// @return 程序退出代码
    fn wait(&self) -> Result<usize, AppError>;
    /// 等待进程结束
    /// @param timeout 等待超时时间
    /// @return 程序退出代码
    fn try_wait(&self, timeout: Duration) -> Result<usize, AppError>;

}

/// 程序启动信息
#[derive(Clone)]
pub struct StartupInfo {
    /// program 需要运行的程序
    pub(crate) program: String,
    /// 启动参数
    pub(crate) args: Vec<String>,
    /// 启动目录
    pub(crate) dir: Option<String>,
    /// 环境变量
    pub(crate) envs: Option<Vec<(String, String)>>,
    /// 运行用户
    pub(crate) user: Option<String>,
    /// 运行所需密码
    pub(crate) password: Option<String>,
    /// 是否使用虚拟终端方式启动
    pub(crate) vt: bool,
    /// 启动周期
    pub(crate) cycles: Vec<Box<&'static dyn StartupCycle>>,
    /// Terminal size
    pub(crate) vt_size: Option<Size>,
    /// 运行时长
    pub(crate) timeout: Option<u64>,
    /// 是否使用特权运行
    pub(crate) privilege: bool
}

struct StartupInfoBuilder {
    info: Mutex<Option<StartupInfo>>
}

impl StartupInfoBuilder {
    /// 创建程序
    /// @param program 需要启动的程序信息
    pub fn of_program<T>(program: T) -> Self where T: AsRef<str>{
        Self {
            info: Mutex::new(Some(StartupInfo {
                program: String::from(program.as_ref()),
                args: Vec::new(),
                dir: None,
                envs: None,
                user: None,
                password: None,
                vt: false,
                privilege: false,
                cycles: Vec::new(),
                vt_size: None,
                timeout: None,
            }))
        }
    }
    /// 添加参数
    /// @param arg 参数信息
    pub fn add_arg<T: ToString>(&self, arg: T) -> &Self{
        let mut info_lock = self.info.lock().unwrap();
        let info = info_lock.as_mut().unwrap();
        info.args.push(arg.to_string());
        self
    }
    /// 添加环境变量
    /// @param key 环境变量key 自持软件环境变量
    /// @param value 环境变量Value
    pub fn add_env<T: ToString>(&self, key: T, value: T) ->&Self {
        let mut info_lock = self.info.lock().unwrap();
        let info = info_lock.as_mut().unwrap();
        if info.envs.is_none() {
            info.envs = Some(Vec::new());
        }
        info.envs.as_mut().unwrap().push((key.to_string(), value.to_string()));
        self
    }

    /// 设置运行目录
    /// @param dir 运行目录
    pub fn dir<T: ToString>(&self, dir: T) -> &Self {
        let mut info_lock = self.info.lock().unwrap();
        let info = info_lock.as_mut().unwrap();
        info.dir = Some(dir.to_string());
        self
    }

    /// 指定用户运行
    /// @param username 用户名
    /// @param password 密码
    pub fn user<T: ToString>(&self, username: Option<T>, password: Option<T>) -> &Self {
        let mut info_lock = self.info.lock().unwrap();
        let info = info_lock.as_mut().unwrap();
        info.user = username.map(|t|{t.to_string()});
        info.password = password.map(|t|{t.to_string()});
        self
    }
    /// 使用虚拟终端运行
    /// @param vt_size 虚拟终端大小
    pub fn use_vt(&self, vt_size: Size) -> &Self {
        let mut info_lock = self.info.lock().unwrap();
        let info = info_lock.as_mut().unwrap();
        info.vt = true;
        info.vt_size = Some(vt_size);
        self
    }
    /// 设置运行超时时间
    /// @param timeout 超时时间 unix 时间戳
    pub fn timeout(&self, timeout: u64) -> &Self {
        let mut info_lock = self.info.lock().unwrap();
        let info = info_lock.as_mut().unwrap();
        info.timeout = Some(timeout);
        self
    }
    /// 添加生命周期
    /// @param cycle
    pub fn add_cycle(&self, cycle: Box<&'static dyn StartupCycle>) -> &Self {
        let mut info_lock = self.info.lock().unwrap();
        let info = info_lock.as_mut().unwrap();
        info.cycles.push(cycle);
        self
    }

    /// 构建
    pub fn build(self) -> StartupInfo {
        self.info.lock().unwrap().take().unwrap()
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn test_build() {
        let build = super::StartupInfoBuilder::of_program("hello");
        build.add_arg("print").add_arg("hello");
        let info = build.build();
        assert_eq!(info.program, "hello");
    }
}