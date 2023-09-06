use crate::app::application::Application;
use std::fmt::{Display, Formatter};

mod action;
mod manager;

pub use self::action::{ActionFunc, ModuleAction, ModuleActionManager, ModuleActionResult};
pub use self::manager::ModuleManager;
/// 模块元数据
pub struct ModuleMeta {
    /// 模块名称
    name: &'static str,
    /// 描述
    desc: &'static str,
}

impl ModuleMeta {
    pub fn new(name: &'static str, desc: &'static str) -> Self {
        Self { name, desc }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn desc(&self) -> &'static str {
        self.desc
    }
}
impl From<(&'static str, &'static str)> for ModuleMeta {
    fn from(value: (&'static str, &'static str)) -> Self {
        ModuleMeta::new(value.0, value.1)
    }
}

impl Display for ModuleMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Module: {} Desc: {}", self.name, self.desc))
    }
}

pub enum ModuleError {}

/// 模块
pub trait Module {
    /// 元数据
    fn meta(&self) -> ModuleMeta;
    /// 初始化模块
    fn on_init(&self, app: Application) -> Result<(), ModuleError> {
        Ok(())
    }
    /// 加载
    fn on_install(&self, app: Application) -> Result<(), ModuleError> {
        Ok(())
    }
    /// 卸载
    fn on_uninstall(&self, app: Application) -> Result<(), ModuleError> {
        Ok(())
    }
    /// 加载
    fn close(&self) {}
    /// 动作
    fn action(&self, _action: ModuleAction) -> ModuleActionResult {
        ModuleActionResult::fail_reason("non")
    }
    /// 克隆
    fn clone(&self) -> Box<dyn Module>;
}
