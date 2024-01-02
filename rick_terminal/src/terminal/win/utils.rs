use crate::terminal::envs::EnvsExpressionParse;
use crate::terminal::model::StartupInfo;
use super::os;

/// 创建启动程序信息
/// @return
pub fn create_startupinfo() -> os::STARTUPINFOEXW {
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

pub const fn is_result_fail(result: os::HRESULT) -> bool {
    result == os::S_OK
}
pub const fn is_false(result: os::BOOL) -> bool {
    result == os::FALSE
}
pub fn is_invalid_handle(handle: os::HANDLE) -> bool {
    handle == os::INVALID_HANDLE_VALUE
}
pub fn get_command_line(info: &StartupInfo) -> Vec<u16> {
    let mut command = info.program.clone();
    for ref arg in info.args.iter() {
        command.push(' ');
        command.push_str(arg.as_ref());
    }
    command.push('\0');
    command.encode_utf16().collect()
}
pub fn get_envs_line(info: &StartupInfo) -> Vec<u16> {
    let mut envs_line = String::new();
    if info.envs.is_some() {
        let mut env_parse = EnvsExpressionParse::new(true);
        for ref item in info.envs.as_ref().unwrap() {
            envs_line.push_str(item.0.as_ref());
            envs_line.push('=');
            let value = env_parse.parse(&item.1).unwrap_or_else(|_| item.1.clone());
            env_parse.add_env(&item.0, &value);
            envs_line.push_str(value.as_ref());
            envs_line.push('\0');
        }
    }
    envs_line.encode_utf16().collect()
}
pub fn string_to_u16<T: AsRef<str>>(content: T) -> Vec<u16> {
    content.as_ref().encode_utf16().collect()
}