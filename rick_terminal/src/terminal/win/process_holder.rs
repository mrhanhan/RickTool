use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::time::Duration;
use rick_core::error::AppError;
use rick_core::utils::ThreadSignal;
use crate::terminal::model::{ProcessHolder, StartupInfo};
use crate::terminal::Size;
use crate::terminal::win::os;
use crate::terminal::win::stdio::Stdio;
use crate::terminal::win::utils;
use crate::terminal::win::utils::{is_false, is_result_fail};
use crate::utils::snow::snow_id;

/// Windows ConPty
#[derive(Clone)]
pub struct ConPTY {
    /// 程序ID
    id: usize,
    /// 进程ID
    process_id: usize,
    /// 线程ID
    thread_id: usize,
    /// 创建程序的启动信息
    info: Arc<Mutex<StartupInfo>>,
    /// windows 进程信息
    win_process_info: Arc<Mutex<os::PROCESS_INFORMATION>>,
    /// windows 启动信息
    win_info: Arc<Mutex<os::STARTUPINFOEXW>>,
    /// Windows 虚拟进程信息
    win_vt_handle: Arc<Mutex<Option<os::HANDLE>>>,
    /// 启动时间
    start_time: u64,
    /// 输出
    stdin: Arc<Mutex<Stdio>>,
    /// 输入
    stdout: Arc<Mutex<Stdio>>,
    /// 线程信号
    signal: Arc<ThreadSignal>,
    /// 程序退出吗
    exit_code: Arc<Mutex<Option<usize>>>,
}

unsafe impl Sync for ConPTY {

}

unsafe impl Send for ConPTY {

}


/// 静态方法
impl ConPTY {
    unsafe fn create(mut info: StartupInfo) -> crate::terminal::Result<Self> {
        let cycles = info.cycles.clone();
        for x in cycles {
            x.on_start(&mut info);
        }
        // 创建管道
        let mut stdin = Stdio::create_anon()?;
        let mut stdout = Stdio::create_anon()?;
        // 进程信息
        let mut win_process_info = os::PROCESS_INFORMATION {
            hProcess: os::INVALID_HANDLE_VALUE,
            hThread: os::INVALID_HANDLE_VALUE,
            dwThreadId: 0,
            dwProcessId: 0,
        };
        let mut win_info: os::STARTUPINFOEXW = utils::create_startupinfo();
        win_info.StartupInfo.cb = std::mem::size_of::<os::STARTUPINFOEXW>() as u32;
        win_info.StartupInfo.dwFlags = os::STARTF_USESTDHANDLES;
        let mut vt_handle: os::HANDLE = os::INVALID_HANDLE_VALUE;
        // 使用vt
        if info.vt {
            let default_vt_size = Size { w: 100, h: 50 };
            let vt_size = info.vt_size.as_ref().unwrap_or(&default_vt_size);
            let vt_size = os::COORD { X: vt_size.w, Y: vt_size.h };
            if is_result_fail(os::CreatePseudoConsole(vt_size.clone(), stdin.read_handle(), stdout.write_handle(), 0, &mut vt_handle)) {
                if !utils::is_invalid_handle(vt_handle) {
                    os::ClosePseudoConsole(vt_handle);
                }
                return Err("创建 虚拟终端失败".into());
            }
            // 初始化线程信息
            // 获取去线程属性列表大小
            let mut size = 0_usize;
            os::InitializeProcThreadAttributeList(os::null_mut(), 1, 0, &mut size);
            win_info.lpAttributeList = os::malloc(size as _) as _;
            // 初始化线程信息
            if is_false(os::InitializeProcThreadAttributeList(win_info.lpAttributeList, 1, 0, &mut size)) {
                os::free(win_info.lpAttributeList as _);
                if !utils::is_invalid_handle(vt_handle) {
                    os::ClosePseudoConsole(vt_handle);
                }
                return Err(format!("初始化线程属性列表失败 Code: {}", os::GetLastError()).into());
            }
            // 更新线程属性信息
            if is_false(os::UpdateProcThreadAttribute(win_info.lpAttributeList, 0, os::PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE as _, vt_handle, std::mem::size_of::<usize>() as _, os::null_mut(), os::null_mut())) {
                os::DeleteProcThreadAttributeList(win_info.lpAttributeList);
                os::free(win_info.lpAttributeList as _);
                if !utils::is_invalid_handle(vt_handle) {
                    os::ClosePseudoConsole(vt_handle);
                }
                return Err(format!("更新线程属性信息失败 Code: {}", os::GetLastError()).into());
            }
        } else {
            win_info.StartupInfo.hStdInput = stdin.read_handle();
            win_info.StartupInfo.hStdOutput = stdout.write_handle();
            win_info.StartupInfo.hStdError = stdout.write_handle();
        }

        let mut command_line = utils::get_command_line(&info);
        let mut envs_line = utils::get_envs_line(&info);
        let env_ptr = if info.envs.is_none() { os::null_mut() as _ } else { envs_line.as_mut_ptr() };
        let mut dir = utils::string_to_u16(match &info.dir {
            None => String::new(),
            Some(_dir) => _dir.to_string()
        });
        let dir_ptr = if info.dir.is_some() { dir.as_mut_ptr() as _ } else { os::null_mut() };
        // 创建进程
        if info.user.is_none() {
            if is_false(os::CreateProcessW(os::null_mut(), command_line.as_mut_ptr(), os::null_mut(), os::null_mut(), os::TRUE,
                                           os::CREATE_UNICODE_ENVIRONMENT | os::EXTENDED_STARTUPINFO_PRESENT, env_ptr as _,
                                           dir_ptr, &mut win_info.StartupInfo, &mut win_process_info)) {
                if !utils::is_invalid_handle(win_process_info.hThread) {
                    os::CloseHandle(win_process_info.hThread);
                }
                if !utils::is_invalid_handle(win_process_info.hProcess) {
                    os::CloseHandle(win_process_info.hProcess);
                }
                os::DeleteProcThreadAttributeList(win_info.lpAttributeList);
                os::free(win_info.lpAttributeList as _);
                if !utils::is_invalid_handle(vt_handle) {
                    os::ClosePseudoConsole(vt_handle);
                }
                return Err(format!("创建进程失败 Code: {}", os::GetLastError()).into());
            }
        } else {
            let mut username = utils::string_to_u16(info.user.as_ref().unwrap());
            let mut password = utils::string_to_u16(info.password.as_ref().unwrap_or(&String::new()));
            if is_false(os::CreateProcessWithLogonW(username.as_mut_ptr() as _, os::null(), password.as_mut_ptr(), os::LOGON_WITH_PROFILE, os::null_mut(), command_line.as_mut_ptr(),
                                                    os::CREATE_UNICODE_ENVIRONMENT | os::EXTENDED_STARTUPINFO_PRESENT, env_ptr as _,
                                                    dir_ptr, &mut win_info.StartupInfo, &mut win_process_info)) {
                if !utils::is_invalid_handle(win_process_info.hThread) {
                    os::CloseHandle(win_process_info.hThread);
                }
                if !utils::is_invalid_handle(win_process_info.hProcess) {
                    os::CloseHandle(win_process_info.hProcess);
                }
                os::DeleteProcThreadAttributeList(win_info.lpAttributeList);
                os::free(win_info.lpAttributeList as _);
                if !utils::is_invalid_handle(vt_handle) {
                    os::ClosePseudoConsole(vt_handle);
                }
                return Err(format!("创建进程失败 Code: {}", os::GetLastError()).into());
            }
        }
        let win_vt_handle = Arc::new(Mutex::new(if info.vt { Some(vt_handle) } else { Option::None }));
        let _info = info.clone();
        let terminal = Self {
            id: snow_id() as usize,
            signal: ThreadSignal::new_arc(),
            exit_code: Arc::new(Mutex::new(None)),
            process_id: win_process_info.dwProcessId as usize,
            thread_id: win_process_info.dwThreadId as usize,
            info: Arc::new(Mutex::new(info)),
            win_process_info: Arc::new(Mutex::new(win_process_info)),
            win_info: Arc::new(Mutex::new(win_info)),
            win_vt_handle,
            start_time: time::SystemTime::now().elapsed().unwrap().as_secs(),
            stdin: Arc::new(Mutex::new(stdin)),
            stdout: Arc::new(Mutex::new(stdout)),
        };
        for x in _info.cycles.clone() {
            x.on_started(&_info, &terminal);
        }
        Ok(terminal)
    }

    fn join(&self) {
        // 等待技术
        let _self = self.clone();
        thread::spawn(move ||{
            unsafe {
                let process_handle  = {_self.win_process_info.lock().unwrap().hProcess.clone()};
                os::WaitForSingleObject(process_handle, 0);
                let mut code = 0;
                os::GetExitCodeProcess(process_handle, &mut code);
                {
                    *_self.exit_code.lock().unwrap() = Some(code as usize);
                }
                // 调用回调函数
                let cycles = {
                    if let Ok(_info) = _self.info.lock() {
                        Some((_info.clone(), _info.cycles.clone()))
                    } else {
                        None
                    }
                };
                if let Some(_info) = cycles {
                    for x in _info.1 {
                        x.on_end(&_info.0, &_self);
                    }
                }
                _self.signal.notify_one();
            }
        });
    }
}

impl ProcessHolder for ConPTY {
    fn id(&self) -> usize {
        self.id
    }

    fn process_id(&self) -> usize {
        self.process_id
    }

    fn thread_id(&self) -> usize {
        self.thread_id
    }

    fn start_time(&self) -> u64 {
        self.start_time
    }

    fn write(&self, data: &[u8]) -> Result<usize, AppError> {
        match self.stdin.lock() {
            Ok(mut _stdin) => {
                match _stdin.write(data) {
                    Ok(_size) => {
                        Ok(_size)
                    }
                    Err(_err) => {
                        Err(AppError::new(500, "写入数据失败"))
                    }
                }
            }
            Err(_) => {
                Err(AppError::new(500, "获取数据锁失败"))
            }
        }
    }

    fn read(&self, data: &mut [u8]) -> Result<usize, AppError> {
        match self.stdout.lock() {
            Ok(mut _stdout) => {
                match _stdout.read(data) {
                    Ok(_size) => {
                        Ok(_size)
                    }
                    Err(_err) => {
                        Err(AppError::new(500, "读取数据失败"))
                    }
                }
            }
            Err(_) => {
                Err(AppError::new(500, "获取数据锁失败"))
            }
        }
    }

    fn resize(&self, size: Size) -> Result<(), AppError> {
        match self.info.lock() {
            Ok(mut _info) => {
                if _info.vt {
                    // 使用虚拟终端
                    // 获取锁
                    if let Ok(mut _win_vt_handle_opt) = self.win_vt_handle.lock() {
                        if let Some(_win_vt_handle) = _win_vt_handle_opt.as_mut() {
                            let result = unsafe { os::ResizePseudoConsole(_win_vt_handle.clone(), os::COORD { X: size.w, Y: size.h }) };
                            if is_result_fail(result) {
                                return Err(AppError::new(500, "更新终端失败"));
                            }
                        }
                    } else {
                        return Err(AppError::new(500, "更新终端失败"));
                    }
                    _info.vt_size = Some(size);
                    Ok(())
                } else {
                    Err(AppError::new(200, "此进程未启用虚拟终端"))
                }
            }
            Err(_err) => {
                Err(AppError::new(500, "获取锁失败"))
            }
        }
    }

    fn kill(&self) -> Result<usize, AppError> {
        todo!()
    }
    fn wait(&self) -> Result<usize, AppError> {
        todo!()
    }
    fn try_wait(&self, timeout: Duration) -> Result<usize, AppError> {
        todo!()
    }
}