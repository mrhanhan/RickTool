use crate::app::application::{Application};

mod action;
mod manager;

pub use self::action::{ModuleResult, ModuleAction};
pub use self::manager::{ModuleManager};
/// 模块元数据
pub struct ModuleMeta {
    /// 模块名称
    name: &'static str,
    /// 描述
    desc: &'static str,
}


pub enum ModuleError {}

/// 模块
pub trait Module {
    /// 元数据
    fn meta(&self) -> ModuleMeta;
    /// 初始化模块
    fn on_init(&self, app: Application) -> Result<(), ModuleError>;
    /// 加载
    fn on_install(&self, app: Application) -> Result<(), ModuleError>;
    /// 卸载
    fn on_uninstall(&self, app: Application) -> Result<(), ModuleError>;
    /// 加载
    fn close(&self);
    /// 动作
    fn action(&self, _action: ModuleAction) -> ModuleResult {
        Err("not action")
    }
    /// 克隆
    fn clone(&self) -> Box<dyn Module>;

}
