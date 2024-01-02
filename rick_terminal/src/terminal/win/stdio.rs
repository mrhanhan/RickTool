use crate::terminal::win::{os, utils};
/// 匿名管道
pub struct AnonymousPipe {
    pub read: os::HANDLE,
    pub write: os::HANDLE
}


impl Drop for AnonymousPipe {
    fn drop(&mut self) {
        if !utils::is_invalid_handle(self.read) {
            unsafe {os::CloseHandle(self.read);}
        }
        if !utils::is_invalid_handle(self.write) {
            unsafe {os::CloseHandle(self.write);}
        }
    }
}
/// 命名管道
pub struct NamedPipe {
    /// 管道名称
    pub name: String,
    pub handle: os::HANDLE
}
impl Drop for NamedPipe {
    fn drop(&mut self) {
        if !utils::is_invalid_handle(self.handle) {
            unsafe {os::CloseHandle(self.handle);}
        }
    }
}
/// 标准输入输出
pub enum Stdio {
    /// 匿名管道 (写入端, 输出端)
    AnonymousPipe(AnonymousPipe),
    NamedPipe(NamedPipe),
}

impl Stdio {
    /// 创建匿名管道
    /// @return Result
    pub fn create_anon() -> crate::terminal::Result<Self> {
        unsafe {
            let mut read_fd = os::INVALID_HANDLE_VALUE;
            let mut write_fd = os::INVALID_HANDLE_VALUE;
            let mut sa_attr = os::SECURITY_ATTRIBUTES {
                nLength: std::mem::size_of::<os::SECURITY_ATTRIBUTES>() as _,
                bInheritHandle: os::TRUE,
                lpSecurityDescriptor: os::null_mut()
            };
            let result_code = os::CreatePipe(&mut read_fd, &mut write_fd, &mut sa_attr, 0);
            // 如果该函数成功，则返回值为非零值。
            if result_code == 0 {
                return Err("Create Pipe field".into());
            }
            Ok(Self::AnonymousPipe(AnonymousPipe{read: read_fd, write: write_fd}))
        }
    }
    ///// 创建名称管道
    // /// @return Result
    // pub fn create_name() -> crate::terminal::Result<Self> {
    //     unsafe{
    //         let mut name = String::new();
    //         loop {
    //             name = format!(
    //                 r"\\.\pipe\rick_tools_.{}.{}",
    //                 os::GetCurrentProcessId(),
    //
    //             );
    //         }
    //         Ok(Self::AnonymousPipe(AnonymousPipe{read: read_fd, write: write_fd}))
    //     }
    // }

}
