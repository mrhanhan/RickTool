use std::io::{Read, Write};
use std::process::Stdio;
use crate::app::actuator::{DataType, DefaultExecuteSession, Executor};
use crate::app::actuator::model::{ShellAction};

const BUFFER_SIZE: usize = 128;

pub struct ShellExecutor {}

impl<'a> Executor<DefaultExecuteSession, ShellAction<'a>> for ShellExecutor {
    fn execute(&self, action: &mut ShellAction) -> Result<DefaultExecuteSession, ()> {
        let session = DefaultExecuteSession::new();
        let mut command = std::process::Command::new(action.shell);
        if let Some(_dir) = action.word_dir {
            command.current_dir(_dir);
        }
        // 处理环境
        if let Some(_envs) = action.envs.as_ref() {
            for _env in _envs {
                command.env(_env.name, _env.value);
            }
        }
        // 参数
        if let Some(_args) = action.args.as_ref() {
            for _arg in _args {
                command.arg(_arg.value);
            }
        }
        let data_consumer = action.data_consumer.clone();
        let mut execute_session = session.clone();
        let _ = std::thread::spawn(move || {
            let stdout_data_consumer = data_consumer.clone();
            let stderr_data_consumer = data_consumer.clone();
            command.stdin(Stdio::piped());
            command.stdout(Stdio::piped());
            command.stderr(Stdio::piped());
            if let Ok(mut _client) = command.spawn() {
                // 写入数据
                if let Some(mut _stdin) = _client.stdin.take() {
                    execute_session.set_consumer(Box::new(move |_data| {
                        _stdin.write(_data).unwrap();
                    }));
                }
                // 读取数据
                let mut stdout = _client.stdout.take().unwrap();
                let mut stderr = _client.stderr.take().unwrap();
                let stdout_async = std::thread::spawn(move || {
                    let mut array_buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
                    loop {
                        if let Ok(_size) = stdout.read(&mut array_buffer) {
                            let _fn = stdout_data_consumer.clone();
                            _fn(&array_buffer[0.._size], DataType::Stdout);
                        } else {
                            break;
                        }
                    }
                });
                let stderr_async = std::thread::spawn(move || {
                    let mut array_buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
                    loop {
                        if let Ok(_size) = stderr.read(&mut array_buffer) {
                            let _fn = stderr_data_consumer.clone();
                            _fn(&array_buffer[0.._size], DataType::StdErr);
                        } else {
                            break;
                        }
                    }
                });
                stdout_async.join().unwrap();
                stderr_async.join().unwrap();
                match _client.wait() {
                    Ok(_code) => {}
                    Err(_) => {}
                };
                execute_session.finish();
            }
        });
        Ok(session)
    }
}


#[cfg(test)]
mod tests {
    use std::io::{stdout, Write};
    use std::sync::Arc;
    use std::thread::sleep;
    use std::time::Duration;
    use crate::app::actuator::Executor;
    use crate::app::actuator::model::{ActionArg, ActionEnv, ShellAction, ShellActionBuilder};
    use crate::app::actuator::shell::ShellExecutor;

    #[test]
    pub fn test_shell() {
        let mut action = ShellAction {
            shell: "java",
            word_dir: None,
            envs: Some(vec![ActionEnv { name: "http_proxy", value: "http://127.0.0.1:7890" }, ActionEnv { name: "https_proxy", value: "http://127.0.0.1:7890" }]),
            args: Some(vec![ActionArg { value: "https://www.google.com" }]),
            data_consumer: Arc::new(|_data, _type| {
                let mut std = stdout();
                std.write(_data).unwrap();
                std.flush().unwrap();
            }),
        };
        let executor = ShellExecutor {};
        // executor.execute(&action);
        // println!("--------");
        action.shell = "curl";
        println!("---- curl -----");
        executor.execute(&mut action).unwrap();
        println!("---- java -----");
        executor.execute(&mut ShellActionBuilder::new("java", Arc::new(|_data, _| {
            let mut std = stdout();
            std.write(_data).unwrap();
            std.flush().unwrap();
        })).arg("--version1").arg("--demo")
            .build()).unwrap();
        sleep(Duration::from_millis(5000));
    }

    #[derive(Debug)]
    struct A{
        a: usize,
        b: i32
    }

    #[test]
    fn mem_ptr() {
        let mut d = 0;
        {
            let a = A {a: 10, b: 31};
            let c = &a as *const A;
            d = c as usize;
            println!("{:p}", &a);
            println!("{:x}", d);
        }
        let a = A {a: 11, b: 30};
        let a = A {a: a.a + 1, b: 30};
        let a = A {a: a.a + 1, b: 30};
        let a = A {a: a.a + 1, b: 30};
        println!("Size: {}", std::mem::size_of_val(&a));
        {
            let c: *mut u64 = d as *mut u64;
            // unsafe {
            //     *c = 12;
            // }
            let a = unsafe {*c};
            println!("{:?}", a);
        }
    }
}