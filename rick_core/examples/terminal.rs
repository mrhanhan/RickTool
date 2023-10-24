use std::io::{Read, stdout, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    let child = Command::new(r"powershell")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();
    if let Ok(mut _child) = child {
        let mut _stdout = _child.stdout.take().unwrap();
        let mut _stdin = _child.stdin.take().unwrap();
        thread::spawn(move || {

            let mut buffer: [u8; 1024]  = [0;1024];
            loop {
                match _stdout.read(&mut buffer) {
                    Ok(_size) => {
                        if _size > 0 {
                            let mut stdout = stdout();
                            stdout.write(&buffer[.._size]);
                            stdout.flush();
                        }
                    }
                    Err(_) => {
                        break
                    }
                }
            }
        });
        _stdin.write("lss".as_bytes()).unwrap();
        _stdin.flush().unwrap();
        thread::sleep(Duration::from_secs(5));
        _stdin.write(&['\u{0008}' as u8]).unwrap();
        _stdin.flush().unwrap();
        _stdin.write(&[13]).unwrap();
        thread::sleep(Duration::from_secs(5));
        _stdin.flush().unwrap();
        thread::sleep(Duration::from_secs(5));
    }
}
