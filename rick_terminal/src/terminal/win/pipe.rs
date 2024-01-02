use crate::terminal::win::os;

#[derive(Clone, Debug)]
pub struct Pipe {
    read_site: os::HANDLE,
    write_site: os::HANDLE,
}

impl Pipe {

    pub unsafe fn create() -> crate::terminal::Result<Self> {
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
        Ok(Self {
            read_site: read_fd,
            write_site: write_fd,
        })
    }
    pub unsafe fn read(&self, buffer: &mut [u8]) -> crate::terminal::Result<usize> {
        let mut read_size = 0_u32;
        if os::ReadFile(self.read_site, buffer.as_mut_ptr() as _, buffer.len() as u32, &mut read_size, os::null_mut()) != 0 {
            Ok(read_size as _)
        } else {
            Err("读取错误".into())
        }
    }

    pub unsafe fn write(&self, buffer: &[u8]) -> crate::terminal::Result<usize> {
        let mut write_size = 0_u32;
        if os::WriteFile(self.write_site, buffer.as_ptr() as _, buffer.len() as _, &mut write_size, os::null_mut()) != 0 {
            Ok(write_size as _)
        } else {
            Err("写入数据错误".into())
        }
    }
    pub fn get_read_handle(&self) -> os::HANDLE {
        self.read_site
    }
    pub fn get_write_handle(&self) -> os::HANDLE {
        self.write_site
    }
}

impl Drop for Pipe {
    fn drop(&mut self) {
        unsafe {
            if self.write_site != os::INVALID_HANDLE_VALUE {
                os::CloseHandle(self.write_site);
                self.write_site = os::INVALID_HANDLE_VALUE;
            }
            if self.read_site != os::INVALID_HANDLE_VALUE {
                os::CloseHandle(self.read_site);
                self.read_site = os::INVALID_HANDLE_VALUE;
            }
        }
    }
}