use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};
use crate::terminal::_impl::conpty::os;
use crate::terminal::_impl::conpty::os::free;
use crate::terminal::_impl::conpty::pipe::Pipe;
use rick_core::utils::ThreadSignal;

/// 启动信息
pub struct StartupInfo {
    /// program 需要运行的程序
    program: String,
    /// 启动参数
    args: Vec<String>,
    /// 启动目录
    dir: Option<String>,
    /// 环境变量
    envs: Vec<(String, String)>,
    /// 是否需要特权运行
    privilege: bool,
    /// 是否虚拟终端
    pty: bool,
    /// callback
    callbacks: Vec<Box<dyn Fn(u32)>>,
    /// use
    stdin_out: bool,
    /// Terminal size
    size: Size
}

impl StartupInfo {
    /// 创建
    pub fn new<T: AsRef<str>>(program: T) -> Self {
        Self {
            program: String::from(program.as_ref()),
            args: Vec::new(),
            dir: None,
            envs: Vec::new(),
            privilege: false,
            pty: false,
            stdin_out: false,
            callbacks: Vec::new(),
            size: Size {w: 100, h: 40}
        }
    }
    /// arg 添加参数
    pub fn arg<T: AsRef<str>>(&mut self, arg: T) -> &mut Self {
        self.args.push(String::from(arg.as_ref()));
        self
    }
    /// env 添加环境变量
    pub fn env<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, val: V) -> &mut Self {
        self.envs.push((String::from(key.as_ref()), String::from(val.as_ref())));
        self
    }
    /// 设置运行目录
    pub fn dir<T: AsRef<str>>(&mut self, dir: T) -> &mut Self {
        self.dir = Some(String::from(dir.as_ref()));
        self
    }
    /// 使用虚拟终端
    pub fn use_pty(&mut self) -> &mut Self {
        self.pty = true;
        self
    }
    /// 使用特权运行
    pub fn use_privilege(&mut self) -> &mut Self {
        self.privilege = true;
        self
    }
    /// 使用输入输出
    pub fn use_stdin_out(&mut self) -> &mut Self {
        self.stdin_out = true;
        self
    }

    pub fn terminal_size(&mut self, w: i16, h: i16) -> &mut Self {
        self.size.w = w;
        self.size.h = h;
        self
    }

    /// 添加回调
    pub fn on_exit<F: Fn(u32) + 'static>(&mut self, callback: F) -> &mut Self {
        self.callbacks.push(Box::new(callback));
        self
    }
}


const fn is_ok(result: os::HRESULT) -> bool {
    result == os::S_OK
}

const fn is_success(result: os::BOOL) -> bool {
    result != os::FALSE
}


fn create_startupinfo() -> os::STARTUPINFOEXW {
    return os::STARTUPINFOEXW {
        StartupInfo: os::STARTUPINFOW {
            cb: 0,
            lpReserved: os::null_mut(),
            lpDesktop: os::null_mut(),
            lpTitle: os::null_mut(),
            dwX: 0,
            dwY: 0,
            dwXSize: 0,
            dwYSize: 0,
            dwXCountChars: 0,
            dwYCountChars: 0,
            dwFillAttribute: 0,
            dwFlags: 0,
            wShowWindow: 0,
            cbReserved2: 0,
            lpReserved2: os::null_mut(),
            hStdInput: os::null_mut(),
            hStdOutput: os::null_mut(),
            hStdError: os::null_mut(),
        },
        lpAttributeList: os::null_mut(),
    };
}

fn is_effective_handle(handle: os::HANDLE) -> bool {
    handle != os::INVALID_HANDLE_VALUE
}

fn string_to_u16<T: AsRef<str>>(content: T) -> Vec<u16> {
    content.as_ref().encode_utf16().collect()
}


impl StartupInfo {
    /// 获取运行命令行
    fn get_command_line(&self) -> Vec<u16> {
        let mut command = self.program.clone();
        for ref arg in self.args.iter() {
            command.push(' ');
            command.push_str(arg.as_ref());
        }
        command.push('\0');
        command.encode_utf16().collect()
    }

    fn get_envs_line(&self) -> Vec<u16> {
        let mut envs_line = String::new();
        for ref item in self.envs.iter() {
            envs_line.push_str(item.0.as_ref());
            envs_line.push('=');
            envs_line.push_str(item.1.as_ref());
            envs_line.push('\0');
        }
        envs_line.encode_utf16().collect()
    }


    /// 创建运行终端
    pub fn create(self) -> Result<Terminal, String> {
        unsafe {
            // 初始化管道
            let mut stdin = Pipe::create()?;
            let mut stdout = Pipe::create()?;
            let _out_ptr = &mut stdout as *mut Pipe as usize;
            // 判断是否需要创建虚拟终端
            let mut pty_handle = os::INVALID_HANDLE_VALUE;
            let pty_size = os::COORD { X: self.size.w, Y: self.size.h };
            let mut info: os::STARTUPINFOEXW = create_startupinfo();
            info.StartupInfo.cb = std::mem::size_of::<os::STARTUPINFOEXW>() as u32;
            if self.stdin_out {info.StartupInfo.dwFlags = os::STARTF_USESTDHANDLES;}
            // 虚拟终端
            if self.pty {
                // 终端大小
                if !is_ok(os::CreatePseudoConsole(pty_size.clone(), stdin.get_read_handle(), stdout.get_write_handle(), 0, &mut pty_handle)) {
                    if is_effective_handle(pty_handle) {
                        os::ClosePseudoConsole(pty_handle);
                    }
                    return Err("创建 虚拟终端失败".into());
                }
                // 初始化线程信息
                // 获取去线程属性列表大小
                let mut size = 0_usize;
                os::InitializeProcThreadAttributeList(os::null_mut(), 1, 0, &mut size);
                info.lpAttributeList = os::malloc(size as _) as _;
                // 初始化线程信息
                if !is_success(os::InitializeProcThreadAttributeList(info.lpAttributeList, 1, 0, &mut size)) {
                    free(info.lpAttributeList as _);
                    if is_effective_handle(pty_handle) {
                        os::ClosePseudoConsole(pty_handle);
                    }
                    return Err(format!("初始化线程属性列表失败 Code: {}", os::GetLastError()).into());
                }
                // 更新线程属性信息
                if !is_success(os::UpdateProcThreadAttribute(info.lpAttributeList, 0, os::PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE as _, pty_handle, std::mem::size_of::<usize>() as _, os::null_mut(), os::null_mut())) {
                    os::DeleteProcThreadAttributeList(info.lpAttributeList);
                    free(info.lpAttributeList as _);
                    if is_effective_handle(pty_handle) {
                        os::ClosePseudoConsole(pty_handle);
                    }
                    return Err(format!("更新线程属性信息失败 Code: {}", os::GetLastError()).into());
                }
            } else {
                info.StartupInfo.hStdInput = stdin.get_read_handle();
                info.StartupInfo.hStdOutput = stdout.get_write_handle();
                info.StartupInfo.hStdError = stdout.get_write_handle();
            }
            // 进程信息
            let mut process_info = os::PROCESS_INFORMATION {
                hProcess: os::INVALID_HANDLE_VALUE,
                hThread: os::INVALID_HANDLE_VALUE,
                dwThreadId: 0,
                dwProcessId: 0,
            };
            // 获取程序信息
            let mut command_line = self.get_command_line();
            // 获取环境信息
            let mut envs_line = self.get_envs_line();
            let env_ptr = if self.envs.is_empty() { os::null_mut() as _ } else { envs_line.as_mut_ptr() };
            // 获取运行命令信息
            let mut dir = string_to_u16(match &self.dir {
                None => String::new(),
                Some(_dir) => _dir.to_string()
            });
            let dir_ptr = if self.dir.is_some() { dir.as_mut_ptr() as _ } else { os::null_mut() };
            // APP
            let mut runas = string_to_u16("runas");
            let application = if self.privilege { runas.as_mut_ptr() as _ } else { os::null_mut() };
            // 创建程序
            if !is_success(os::CreateProcessW(application, command_line.as_mut_ptr(),
                                              os::null_mut(), os::null_mut(), os::TRUE,
                                              os::CREATE_UNICODE_ENVIRONMENT | os::EXTENDED_STARTUPINFO_PRESENT,
                                              env_ptr as _, dir_ptr,
                                              &mut info.StartupInfo,
                                              &mut process_info)) {
                if self.pty {
                    os::DeleteProcThreadAttributeList(info.lpAttributeList);
                    free(info.lpAttributeList as _);
                }
                if is_effective_handle(process_info.hThread) {
                    os::CloseHandle(process_info.hThread);
                }
                if is_effective_handle(process_info.hProcess) {
                    os::CloseHandle(process_info.hProcess);
                }
                if is_effective_handle(pty_handle) {
                    os::ClosePseudoConsole(pty_handle);
                }
                return Err(format!("创建进程失败 Code: {}", os::GetLastError()).into());
            }
            let mut terminal = Terminal {
                stdin: Arc::new(Mutex::new(stdin)),
                stdout: Arc::new(Mutex::new(stdout)),
                size: Arc::new(Mutex::new(pty_size)),
                pty_handle,
                process_info,
                info, stdin_out: self.stdin_out,
                use_pty: self.pty,
                exit_code: Arc::new(Mutex::new(0)),
                running_status: Arc::new(Mutex::new(true)),
                drop_status: Arc::new(Mutex::new(false)),
                callbacks: Arc::new(self.callbacks),
                signal: ThreadSignal::new_arc(),
            };
            Terminal::start_wait_exit(terminal.clone());
            Ok(terminal)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub w: i16,
    pub h: i16,
}

/// ConPTY
#[derive(Clone)]
pub struct Terminal {
    /// stdin 写入数据段
    stdin: Arc<Mutex<Pipe>>,
    /// stdout 输出数据段
    stdout: Arc<Mutex<Pipe>>,
    /// 虚拟终端大小
    size: Arc<Mutex<os::COORD>>,
    /// pty handler
    pty_handle: os::HANDLE,
    /// process info
    process_info: os::PROCESS_INFORMATION,
    /// 启动信息
    info: os::STARTUPINFOEXW,
    /// 是否使用了Pty
    use_pty: bool,
    /// drop 状态
    drop_status: Arc<Mutex<bool>>,
    /// 终端状态 true, 运行中，false 已停止
    running_status: Arc<Mutex<bool>>,
    /// 退出状态码
    exit_code: Arc<Mutex<u32>>,
    /// callback
    callbacks: Arc<Vec<Box<dyn Fn(u32)>>>,
    /// use
    stdin_out: bool,
    /// 信号
    signal: Arc<ThreadSignal>,
}

unsafe impl Sync for Terminal {

}

unsafe impl Send for Terminal {

}
impl Terminal {
    pub fn get_size(&self) -> Size {
        let size = self.size.lock().unwrap().clone();
        Size {
            w: size.X,
            h: size.Y,
        }
    }

    pub fn get_process_id(&self) -> i32 {
        self.process_info.dwProcessId as _
    }

    pub fn get_running_status(&self) -> bool {
        *(self.running_status.clone().lock().unwrap())
    }
    pub fn get_exit_code(&self) -> u32 {
        *(self.exit_code.clone().lock().unwrap())
    }

    pub fn kill(&self){
        if !self.get_running_status() {
            return;
        }
        unsafe {
            os::TerminateProcess(self.process_info.hProcess, 1);
        }
    }

    pub fn read(&self, buffer: &mut [u8]) -> crate::terminal::Result<usize> {
        if !self.get_running_status() || !self.stdin_out {
            return Ok(0);
        }
        unsafe {
            self.stdout.lock().unwrap().read(buffer)
        }
    }

    pub fn write(&self, buffer: &[u8]) -> crate::terminal::Result<usize> {
        if !self.get_running_status() || !self.stdin_out {
            return Ok(0);
        }
        unsafe {
            self.stdin.lock().unwrap().write(buffer)
        }
    }
    /// 设置终端大小
    pub fn set_size(&mut self, size: Size) {
        if self.get_running_status() && self.use_pty {
            let mut self_size = self.size.lock().unwrap();
            (*self_size).X = size.w;
            (*self_size).Y = size.h;
            unsafe {
                os::ResizePseudoConsole(self.pty_handle, self_size.clone());
            }
        }
    }

    fn start_wait_exit(terminal: Terminal) {
        let signal = terminal.signal.clone();
        thread::spawn(move || {
            let _self = terminal;
            unsafe {
                // 等待线程结束
                os::WaitForSingleObject(_self.process_info.hThread, 0xFFFFFFFF);
                *(_self.running_status.lock().unwrap()) = false;
                let mut code = 0;
                if !is_success(os::GetExitCodeProcess(_self.process_info.hProcess, &mut code)) {
                    println!("获取程序状态码失败");
                }
                *(_self.exit_code.lock().unwrap()) = code;
                signal.notify_all();
                for i in 0.._self.callbacks.len() {
                    if let Some(_callback) = _self.callbacks.get(i) {
                        _callback(code);
                    }
                }
                _self.close();
            }
        });
    }


    /// 等待进程执行结束
    pub fn wait(&self) -> crate::terminal::Result<u32> {
        if self.get_running_status() {
            self.signal.wait()
        }
        Ok(self.get_exit_code())
    }

    pub fn close(&self) {
        println!("Terminal 关闭");
        let mut status = self.drop_status.lock().unwrap();
        if *status {
            return;
        }
        *status = true;
        unsafe {
            {
                self.stdin.lock().unwrap().close();
            }
            {
                self.stdout.lock().unwrap().close();
            }
            if self.use_pty {
                os::DeleteProcThreadAttributeList(self.info.lpAttributeList);
                free(self.info.lpAttributeList as _);
            }
            if is_effective_handle(self.process_info.hThread) {
                os::CloseHandle(self.process_info.hThread);
            }
            if is_effective_handle(self.process_info.hProcess) {
                os::CloseHandle(self.process_info.hProcess);
            }
            if is_effective_handle(self.pty_handle) {
                os::ClosePseudoConsole(self.pty_handle);
            }
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {}
}

