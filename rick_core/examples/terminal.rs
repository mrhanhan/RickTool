use std::io::Read;
use std::process::{Command, Stdio};

fn main() {
    let child = Command::new("powershell")
        // .envs(std::env::args())
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .spawn();
    if let Ok(mut _child) = child {
        let mut _stdout = _child.stdout.take().unwrap();
        let mut buffer: [u8; 1024]  = [0;1024];


        loop {
            match _stdout.read(&mut buffer) {
                Ok(_size) => {
                    if _size > 0 {
                        println!("{}", String::from_utf8_lossy(&buffer[.._size]));
                    }
                }
                Err(_) => {
                    break
                }
            }
        }
    }
}
