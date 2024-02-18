use std::io::{Read, Write};
use crate::terminal::win::{os, utils};
use crate::utils::snow::{snow_hex};

/// 匿名管道
pub struct AnonymousPipe {
    pub read: os::HANDLE,
    pub write: os::HANDLE,
}


impl Drop for AnonymousPipe {
    fn drop(&mut self) {
        if !utils::is_invalid_handle(self.read) {
            unsafe { os::CloseHandle(self.read); }
        }
        if !utils::is_invalid_handle(self.write) {
            unsafe { os::CloseHandle(self.write); }
        }
    }
}

/// 命名管道
pub struct NamedPipe {
    /// 管道名称
    pub name: String,
    pub handle: os::HANDLE,
    /// 是否是server 端
    pub server: bool,
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        if !utils::is_invalid_handle(self.handle) {
            unsafe {
                if self.server {
                    os::DisconnectNamedPipe(self.handle);
                }
                os::CloseHandle(self.handle);
            }
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
    /// 如果是名称管道获取管道名称
    /// @return 管道名称
    pub fn name(&self) -> Option<String> {
        if let Self::NamedPipe(ref _pip) = self {
            return Some(_pip.name.clone());
        }
        None
    }

    /// 获取读取端的句柄
    pub fn read_handle(&self) -> os::HANDLE {
        match self {
            Stdio::AnonymousPipe(ref _pipe) => {
                _pipe.read
            }
            Stdio::NamedPipe(ref _pipe) => {
                _pipe.handle
            }
        }
    }
    /// 获取写入端的句柄
    pub fn write_handle(&self) -> os::HANDLE {
        match self {
            Stdio::AnonymousPipe(ref _pipe) => {
                _pipe.write
            }
            Stdio::NamedPipe(ref _pipe) => {
                _pipe.handle
            }
        }
    }

    /// 创建匿名管道
    /// @return Result
    pub fn create_anon() -> crate::terminal::Result<Self> {
        unsafe {
            let mut read_fd = os::INVALID_HANDLE_VALUE;
            let mut write_fd = os::INVALID_HANDLE_VALUE;
            let mut sa_attr = os::SECURITY_ATTRIBUTES {
                nLength: std::mem::size_of::<os::SECURITY_ATTRIBUTES>() as _,
                bInheritHandle: os::TRUE,
                lpSecurityDescriptor: os::null_mut(),
            };
            let result_code = os::CreatePipe(&mut read_fd, &mut write_fd, &mut sa_attr, 0);
            // 如果该函数成功，则返回值为非零值。
            if result_code == 0 {
                return Err("Create Pipe field".into());
            }
            Ok(Self::AnonymousPipe(AnonymousPipe { read: read_fd, write: write_fd }))
        }
    }
    /// 随机一个管道名称
    /// @return
    pub fn random_pipe_name() -> String {
        format!(
            r"\\.\pipe\rick_tools_uac_.{}.{}", os::current_process_id(), snow_hex()
        )
    }

    // 创建名称管道
    /// @return Result
    pub fn create_name(name: String) -> crate::terminal::Result<Self> {
        unsafe {
            ;
            let buffer_size = 1024 * 1024;
            let os_name = utils::string_to_u16(name.clone());
            let handle = os::CreateNamedPipeW(os_name.as_ptr(), os::PIPE_ACCESS_DUPLEX,
                                              os::PIPE_TYPE_BYTE | os::PIPE_REJECT_REMOTE_CLIENTS | os::PIPE_WAIT,
                                              255, buffer_size, buffer_size, 10000, os::null_mut());
            if utils::is_invalid_handle(handle) {
                return Err(format!("管道创建失败:{}", os::GetLastError()));
            }
            if utils::is_false(os::ConnectNamedPipe(handle, os::null_mut())) {
                os::CloseHandle(handle);
                return Err(format!("连接创建失败:{}", os::GetLastError()));
            }
            Ok(Self::NamedPipe(NamedPipe { handle, name, server: true }))
        }
    }
    /// 连接管道
    /// @param name 管道名称
    pub fn connect_name(name: String) -> crate::terminal::Result<Self> {
        unsafe {
            let os_name = utils::string_to_u16(name.clone());
            if utils::is_false(os::WaitNamedPipeW(os_name.as_ptr(), 10000)) {
                return Err(format!("管道连接失败, 等待超时:{}", os::GetLastError()));
            }
            let handle = os::CreateFileW(os_name.as_ptr(), os::GENERIC_READ | os::GENERIC_WRITE, os::FILE_SHARE_READ | os::FILE_SHARE_WRITE,
                                         os::null_mut(), os::OPEN_EXISTING,
                                         os::FILE_ATTRIBUTE_NORMAL, os::null_mut());
            if utils::is_invalid_handle(handle) {
                return Err(format!("管道连接失败:{}", os::GetLastError()));
            }

            Ok(Self::NamedPipe(NamedPipe { handle, name, server: false }))
        }
    }
}

impl Read for Stdio {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Stdio::AnonymousPipe(ref _pipe) => {
                let mut read_size = 0_u32;
                unsafe {
                    if os::ReadFile(_pipe.read, buf.as_mut_ptr() as _, buf.len() as u32, &mut read_size, os::null_mut()) != 0 {
                        Ok(read_size as _)
                    } else {
                        Err(std::io::Error::last_os_error())
                    }
                }
            }
            Stdio::NamedPipe(ref _pipe) => {
                let mut read_size = 0_u32;
                unsafe {
                    if os::ReadFile(_pipe.handle, buf.as_mut_ptr() as _, buf.len() as u32, &mut read_size, os::null_mut()) != 0 {
                        Ok(read_size as _)
                    } else {
                        Err(std::io::Error::last_os_error())
                    }
                }
            }
        }
    }
}

impl Write for Stdio {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Stdio::AnonymousPipe(ref _pipe) => {
                unsafe {
                    let mut write_size = 0_u32;
                    if os::WriteFile(_pipe.write, buf.as_ptr() as _, buf.len() as _, &mut write_size, os::null_mut()) != 0 {
                        Ok(write_size as _)
                    } else {
                        Err(std::io::Error::last_os_error())
                    }
                }
            }
            Stdio::NamedPipe(ref _pipe) => {
                unsafe {
                    let mut write_size = 0_u32;
                    if os::WriteFile(_pipe.handle, buf.as_ptr() as _, buf.len() as _, &mut write_size, os::null_mut()) != 0 {
                        Ok(write_size as _)
                    } else {
                        Err(std::io::Error::last_os_error())
                    }
                }
            }
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use crate::utils::io::protocol::Decode;
    use super::Stdio;

    #[test]
    fn test_anno() {
        let mut stdio = Stdio::create_anon().unwrap();
        stdio.write("hello".as_bytes()).unwrap();
        let mut buf: [u8; 1024] = [0; 1024];
        let size = stdio.read(&mut buf).unwrap();
        println!("Read:{}", String::from_utf8_lossy(&buf[0..size]));
    }

    #[test]
    fn test_name() {
        let mut name = Stdio::random_pipe_name();
        let stdout_name = name.clone();
        let stdout_name1 = name.clone();
        let t1 = thread::spawn(move || {
            println!("开始打开管道:{}", stdout_name);
            let mut stdio_write = Stdio::create_name(stdout_name.clone()).unwrap();
            println!("开始链接管道:{}", stdio_write.name().unwrap());
            let mut count = 0;
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            stdio_write.write(crate::utils::io::protocol::Encode::encode("Hello World").as_slice()).unwrap();
            sleep(Duration::from_secs(10))
        });
        let t2 = thread::spawn(move || {
            sleep(Duration::from_secs(1));
            let mut stdio_read = Stdio::connect_name(name.clone()).unwrap();
            let mut buf: [u8; 1024] = [0; 1024];
            let mut decode = Decode::new(Box::new(|data: &[u8]| {
                println!("Read:{}", String::from_utf8_lossy(&data));
            }));
            let size = stdio_read.read(&mut buf).unwrap();
            decode.decode(&buf[0..size]);
            let size = stdio_read.read(&mut buf).unwrap();
            decode.decode(&buf[0..size]);

        });

        t1.join().expect("TODO: panic message");
        t2.join().expect("TODO: panic message");
    }
}