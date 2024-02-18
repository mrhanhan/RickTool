
pub use winapi::um::winnt::{HANDLE, LPCWSTR, HRESULT, GENERIC_READ, GENERIC_WRITE, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE};
pub use winapi::shared::minwindef::{BOOL};
pub use winapi::um::winbase::{STARTUPINFOEXW, EXTENDED_STARTUPINFO_PRESENT, CREATE_UNICODE_ENVIRONMENT, CREATE_NEW_CONSOLE,
                              STARTF_USESTDHANDLES, CreateProcessWithLogonW, PIPE_ACCESS_DUPLEX, PIPE_ACCESS_INBOUND, PIPE_ACCESS_OUTBOUND, LOGON_WITH_PROFILE,
                              FILE_FLAG_FIRST_PIPE_INSTANCE, FILE_FLAG_WRITE_THROUGH, PIPE_TYPE_BYTE, PIPE_WAIT, PIPE_NOWAIT, PIPE_REJECT_REMOTE_CLIENTS, PIPE_UNLIMITED_INSTANCES
};
pub use winapi::um::handleapi::{INVALID_HANDLE_VALUE, CloseHandle};
pub use winapi::um::fileapi::{ReadFile, WriteFile, CreateFileW, OPEN_EXISTING, };
pub use winapi::um::errhandlingapi::{GetLastError};
pub use winapi::um::namedpipeapi::{CreatePipe, CreateNamedPipeW, ConnectNamedPipe, DisconnectNamedPipe, WaitNamedPipeW};
pub use winapi::um::shellapi::ShellExecuteW;
pub use winapi::um::minwinbase::{SECURITY_ATTRIBUTES};
pub use winapi::um::processthreadsapi::{PROC_THREAD_ATTRIBUTE_LIST, GetCurrentProcessId, PROCESS_INFORMATION, STARTUPINFOW, InitializeProcThreadAttributeList,
                                        GetExitCodeProcess, DeleteProcThreadAttributeList, TerminateProcess,
                                        UpdateProcThreadAttribute, CreateProcessAsUserW, CreateProcessW, OpenProcessToken, GetCurrentProcess};
pub use winapi::um::synchapi::{WaitForSingleObject};
pub use winapi::um::wincontypes::{COORD};
pub use winapi::ctypes::{c_void};
pub use winapi::um::consoleapi::{ResizePseudoConsole, ClosePseudoConsole, CreatePseudoConsole};

pub use std::ptr::{null_mut, null};

pub const PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE: u32 = 131094;

pub const S_OK: i32 = 0;
pub const S_FALSE: i32 = 1;

pub const FALSE: i32 = 0;
pub const TRUE: i32 = 1;

extern "system" {
    /// 分配内存
    pub fn malloc(size: u32) -> *mut c_void;
    /// 释放内存
    pub fn free(ptr: *mut c_void);
}


pub fn current_process_id() -> u32 {
    unsafe {GetCurrentProcessId()}
}