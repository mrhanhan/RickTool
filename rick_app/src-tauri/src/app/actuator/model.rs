use std::sync::Arc;
use crate::app::actuator::DataType;


type Consumer = Arc<dyn Fn(&[u8], DataType) + Send + Sync>;

pub trait Action {
    /// 模型类型
    fn model_type(&self) -> &'static str;
}
/// 动作环境
#[derive(Debug, Clone, Copy)]
pub struct ActionEnv<'a> {
    /// 环境名称
    pub name: &'a str,
    /// 环境值
    pub value: &'a str
}

/// 动作参数
pub struct ActionArg<'a>{
    /// 环境值
    pub value: &'a str
}

/// Shell 执行程序
pub struct ShellAction<'a> {
    /// 需要执行的Shell
    pub shell: &'a str,
    /// 工作目录
    pub word_dir: Option<&'a str>,
    /// 环境配置
    pub envs: Option<Vec<ActionEnv<'a>>>,
    /// Shell 参数
    pub args: Option<Vec<ActionArg<'a>>>,
    /// 数据消费者
    pub data_consumer: Consumer
}

pub struct ShellActionBuilder<'a>(ShellAction<'a>);

impl<'a> ShellActionBuilder<'a> {

    pub fn new(program: &'a str, consumer: Consumer) -> Self {
        Self(ShellAction {
            shell: program,
            word_dir: None,
            args: None,
            envs: None,
            data_consumer: consumer.into(),
        })
    }

    pub fn arg(mut self, value: &'a str) -> Self {
        match self.0.args.as_mut() {
            None => {
                self.0.args = Some(vec![ActionArg {value}])
            },
            Some(mut _vec) => {
                _vec.push(ActionArg {value});
            }
        }
        self
    }
    pub fn env(mut self, name: &'a str, value: &'a str) -> Self {
        match self.0.envs.as_mut() {
            None => {
                self.0.envs = Some(vec![ActionEnv {name, value}])
            },
            Some(mut _vec) => {
                _vec.push(ActionEnv {name, value});
            }
        }
        self
    }
    pub fn work_dir(mut self, pwd: &'a str) -> Self {
        match self.0.word_dir.as_mut() {
            None => {
                self.0.word_dir = Some(pwd)
            },
            Some(_) => {
                self.0.word_dir = Some(pwd)
            }
        }
        self
    }
    pub fn build(self) -> ShellAction<'a> {
        self.0
    }
}

impl<'a> Action for ShellAction<'a> {
    fn model_type(&self) -> &'static str {
        "shell"
    }
}
