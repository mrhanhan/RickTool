use crate::terminal::model::{ProcessHolder, StartupInfo};

mod os;
mod process_holder;
mod utils;
mod pipe;
mod stdio;

/// 创建进程
/// @param info 进程信息
/// @return Result 返回创建信息
pub fn create_process(info: StartupInfo) -> crate::terminal::Result<Box<dyn ProcessHolder>> {

    Err("".to_string())
}