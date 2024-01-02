use std::os::windows::io::IntoRawHandle;
use std::sync::Mutex;
use crate::terminal::model::StartupInfo;
use crate::terminal::Size;
use crate::terminal::win::os;
use crate::terminal::win::pipe::Pipe;
use crate::terminal::win::utils;
use crate::terminal::win::utils::{is_false, is_result_fail};

/// Windows ConPty
pub struct ConPTY {
    /// 创建程序的启动信息
    info: Mutex<StartupInfo>,
    /// windows 进程信息
    win_process_info: Mutex<os::STARTUPINFOEXW>,
    /// windows 启动信息
    win_info: Mutex<os::STARTUPINFOEXW>,
    /// Windows 虚拟进程信息
    win_vt_handle: Mutex<Option<os::HANDLE>>,
    /// 启动时间
    start_time: u64,
    /// 输出
    stdin: Mutex<Pipe>,
    /// 输入
    stdout: Mutex<Pipe>
}

/// 静态方法
impl ConPTY {

    unsafe fn create(info: StartupInfo) -> crate::terminal::Result<Self> {
        // 创建管道
        let mut stdin = Pipe::create()?;
        let mut stdout = Pipe::create()?;
        let mut win_vt_handle: Option<os::HANDLE> = Option::None;
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
        // 使用vt
        if info.vt {
            let mut vt_handle: os::HANDLE = os::INVALID_HANDLE_VALUE;
            let default_vt_size = Size { w: 100, h: 50 };
            let vt_size = info.vt_size.as_ref().unwrap_or(&default_vt_size);
            let vt_size = os::COORD { X: vt_size.w, Y: vt_size.h };
            // let handle = stdin.into_raw_handle();
            if is_result_fail(os::CreatePseudoConsole(vt_size.clone(), stdin.get_read_handle(), stdout.get_write_handle(), 0, &mut vt_handle)) {
                if ! utils::is_invalid_handle(vt_handle) {
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
                if ! utils::is_invalid_handle(vt_handle) {
                    os::ClosePseudoConsole(vt_handle);
                }
                return Err(format!("初始化线程属性列表失败 Code: {}", os::GetLastError()).into());
            }
            // 更新线程属性信息
            if is_false(os::UpdateProcThreadAttribute(win_info.lpAttributeList, 0, os::PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE as _, vt_handle, std::mem::size_of::<usize>() as _, os::null_mut(), os::null_mut())) {
                os::DeleteProcThreadAttributeList(win_info.lpAttributeList);
                os::free(win_info.lpAttributeList as _);
                if ! utils::is_invalid_handle(vt_handle) {
                    os::ClosePseudoConsole(vt_handle);
                }
                return Err(format!("更新线程属性信息失败 Code: {}", os::GetLastError()).into());
            }
        }  else {
            win_info.StartupInfo.hStdInput = stdin.get_read_handle();
            win_info.StartupInfo.hStdOutput = stdout.get_write_handle();
            win_info.StartupInfo.hStdError = stdout.get_write_handle();
        }
        let mut command_line = utils::get_command_line(&info);
        let mut envs_line = utils::get_envs_line(&info);
        let env_ptr = if info.envs.is_none() { os::null_mut() as _ } else { envs_line.as_mut_ptr() };
        let mut dir = utils::string_to_u16(match &info.dir {
            None => String::new(),
            Some(_dir) => _dir.to_string()
        });
        let dir_ptr = if info.dir.is_some() { dir.as_mut_ptr() as _ } else { os::null_mut() };
        return Err("创建 虚拟终端失败".into());
    }
}
