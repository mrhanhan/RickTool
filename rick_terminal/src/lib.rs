pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::ffi::c_void;
    use std::ptr;
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        let mut inPipeRead = INVALID_HANDLE_VALUE;
        let mut inPipeWrite = INVALID_HANDLE_VALUE;
        let mut outPipeRead = INVALID_HANDLE_VALUE;
        let mut outPipeWrite = INVALID_HANDLE_VALUE;
        let result = unsafe { winapi::um::namedpipeapi::CreatePipe(&mut inPipeRead, &mut inPipeWrite, ptr::null_mut(), 0) };
        if result != 0 {
            return;
        }
        let result = unsafe { winapi::um::namedpipeapi::CreatePipe(&mut outPipeRead, &mut outPipeWrite, ptr::null_mut(), 0) };
        if result != 0 {
            return;
        }
        let coord = winapi::um::wincontypes::COORD {X: 10_i16, Y: 20_i16};
        let mut console = INVALID_HANDLE_VALUE;
        let result = unsafe {winapi::um::consoleapi::CreatePseudoConsole(coord, inPipeRead, outPipeWrite, 0, &mut console)};
        if result != 0 {
            return;
        }
    }
}
