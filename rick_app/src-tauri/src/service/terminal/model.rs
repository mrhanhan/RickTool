use std::io::{Read, Write};
use std::process::{Command, Stdio, ChildStdout, ChildStdin, Child};
use std::sync::{Arc, Mutex, RwLock};
use std::{env, thread};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::global::RickWindow;

/// 终端
#[derive(Clone)]
pub struct SimpleTerminal {
    /// Terminal
    name: String,
    /// ID
    pub id: String,
    /// 标准输出
    stdin: Arc<Mutex<Option<ChildStdin>>>,
    /// 执行的程序
    child: Arc<Mutex<Option<Child>>>,
    /// 数据
    data: Arc<Mutex<Vec<u8>>>,
    /// 当前窗口
    window: RickWindow,
}

#[derive(Clone, Serialize)]
pub struct SimpleTerminalData {
    id: String,
    output_type: u8,
    data: Vec<u8>,
}

impl SimpleTerminal {
    pub fn new(id: String, name: String, window: RickWindow) -> Self {
        Self {
            id,
            name,
            window,
            data: Arc::new(Mutex::new(Vec::new())),
            stdin: Arc::new(Mutex::new(None)),
            child: Arc::new(Mutex::new(None)),
        }
    }
}

impl SimpleTerminal {
    pub fn start(&self) {
        let mut command = Command::new("powershell.exe");
        command
            .envs(env::vars_os())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped());
        let stdin = self.stdin.clone();
        let child = self.child.clone();
        if let Ok(mut _client) = command.spawn() {
            let _stdin = _client.stdin.take().unwrap();
            let _stderr = _client.stderr.take().unwrap();
            let _stdout = _client.stdout.take().unwrap();
            if let Ok(mut _stdin_guard) = stdin.lock() {
                *_stdin_guard = Some(_stdin);
            }
            self.read_output(_stdout);
            self.read_output(_stderr);
            if let Ok(mut _child_guard) = child.lock() {
                *_child_guard = Some(_client);
            }
        } else {
            println!("启动错误");
        }
    }

    /// 写入数据
    pub fn write(&self, data: &[u8]) {
        if let Ok(mut _stdin) = self.stdin.lock() {
            if let Some(_stdin) = _stdin.as_mut() {
                let _ = _stdin.write(data);
            }
        }
        if let Ok(mut _data) = self.data.lock() {
            // let _ = _data.write(data);
        }
        // self.update(1, data);
    }

    /// 从指定位置开始读取
    pub fn read(&self, offset: usize) -> Vec<u8> {
        if let Ok(mut _data) = self.data.lock() {
            let _data = &_data.as_slice()[offset..];
            println!("READ: {}", String::from_utf8_lossy(_data));
            return _data.into();
        }
        Vec::new()
    }


    pub fn kill(self) {
        if let Ok(mut _child) = self.child.lock() {
            if let Some(_child) = _child.as_mut() {
                let _ = _child.kill();
            }
        }
    }

    fn read_output<R: Read + Send + 'static>(&self, mut read: R) {
        let this = self.clone();
        let _ = thread::spawn(move || {
            let mut buffer: [u8; 1024] = [0; 1024];
            loop {
                if let Ok(_size) = read.read(&mut buffer) {
                    if _size == 0 {
                        continue;
                    }
                    let _buffer_data = &buffer[.._size];
                    if let Ok(mut _data) = this.data.lock() {
                        let _ = _data.write(_buffer_data);
                    }
                    println!("DATA: {}", String::from_utf8_lossy(_buffer_data));
                    this.update(1, _buffer_data);
                } else {
                    break;
                }
            }
        });
    }

    fn update(&self, output_type: u8, data: &[u8]) {
        let _ = self.window.emit("terminal:data", SimpleTerminalData {
            id: self.id.clone(),
            output_type,
            data: data.into()
        });
    }
}

impl Serialize for SimpleTerminal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("SimpleTerminal", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("id", &self.id)?;
        state.end()
    }
}