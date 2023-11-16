use std::io::{Read, stdin, stdout, Write};
use std::thread;
use rick_terminal::terminal::{StartupInfo, Terminal, Pipe};

#[test]
pub fn test_terminal() {
    let mut info = StartupInfo::new("powershell");
    let terminal = info.create().unwrap();
    let t1 = terminal.clone();
    thread::spawn(move ||{
        let mut buf: [u8; 512] = [0; 512];
        loop {
            let size = t1.read(&mut buf).unwrap();
            if size > 0 {
                stdout().write(&buf[..size as _]);
                stdout().flush();
            } else {
                println!("无内容");
                break
            }
        }
    });
    let mut buf: [u8; 512] = [0; 512];
    loop {
        let size = stdin().read(&mut buf).unwrap();
        terminal.write(&buf[..size]);
    }
    // terminal.write("start cmd\r\n".as_bytes());
    let _ = terminal.wait();
}