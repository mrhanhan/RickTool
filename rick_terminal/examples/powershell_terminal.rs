use std::io::{Read, stdin, stdout, Write};
use std::thread;
use std::time::Duration;
use rick_terminal::terminal::{StartupInfo, Terminal, Pipe};


fn main() {
    let mut info = StartupInfo::new("powershell");
    info.use_pty();
    let terminal = info.create().unwrap();
    let t1 = terminal.clone();
    thread::spawn(move ||{
        let mut buf: [u8; 512] = [0; 512];
        loop {
            let size = t1.read(&mut buf).unwrap();
            if size > 0 {
                stdout().write(&buf[..size as _]);
                stdout().flush();
            }
        }
    });
    // terminal.write("start cmd\r\n".as_bytes());
    terminal.write("dir\r\n".as_bytes());
    thread::sleep(Duration::from_secs(2));
    terminal.write("cls\r\n".as_bytes());
    thread::sleep(Duration::from_secs(2));
    terminal.write("exit\r\n".as_bytes());
    let _ = terminal.wait();
}