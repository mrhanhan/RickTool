use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::{env, thread};
use log::{error, info, log};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use rick_terminal::terminal::{StartupInfo, Terminal};
use crate::global::RickWindow;

/// 终端
#[derive(Clone)]
pub struct SimpleTerminal {
    /// Terminal
    name: String,
    /// ID
    pub id: String,
    /// 标准输出
    terminal: Arc<Mutex<Option<Terminal>>>,
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
            terminal: Arc::new(Mutex::new(Option::None))
        }
    }
}

impl SimpleTerminal {
    pub fn start(&self) {
        info!("启动 Powershell");
        let mut powershell = StartupInfo::new("powershell");
        powershell.use_pty().use_stdin_out();
        if let Ok(mut _client) = powershell.create() {
            self.read_output(_client.clone());
            if let Ok(mut _child_guard) = self.terminal.lock() {
                *_child_guard = Some(_client);
            }
        } else {
            error!("启动错误");
        }
    }

    /// 写入数据
    pub fn write(&self, data: &[u8]) {
        info!("写入数据:{}", data.len());
        if let Ok(mut _stdin) = self.terminal.lock() {
            if let Some(_stdin) = _stdin.as_mut() {
                let _ = _stdin.write(data);
            }
        }
    }

    /// 从指定位置开始读取
    pub fn read(&self, offset: usize) -> Vec<u8> {
        if let Ok(mut _data) = self.data.lock() {
            let _data = &_data.as_slice()[offset..];
            info!("READ: {}", String::from_utf8_lossy(_data));
            return _data.into();
        }
        Vec::new()
    }


    pub fn kill(self) {
        if let Ok(mut _child) = self.terminal.lock() {
            if let Some(_child) = _child.as_mut() {
                let _ = _child.kill();
            }
        }
    }

    fn read_output(&self, mut read: Terminal) {
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
                    this.update(1, _buffer_data);
                } else {
                    break;
                }
            }
        });
    }

    fn update(&self, output_type: u8, data: &[u8]) {
        info!("输入: {}: {}\n------------------------------\n{:?}\n------------------------------\n{}\n------------------------------", output_type, data.len(), data, String::from_utf8_lossy(data));
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