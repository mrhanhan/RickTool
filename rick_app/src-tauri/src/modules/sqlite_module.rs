use crate::app::application::Application;
use crate::app::module::{Module, ModuleAction, ModuleActionManager, ModuleActionResult, ModuleError, ModuleMeta};
use crate::define_event;

#[derive(Clone)]
pub struct SqliteModule(ModuleActionManager);

impl SqliteModule {
    pub fn new() -> Self{
        Self(ModuleActionManager::new())
    }
}

define_event!(SqliteAction => Query,Execute);

impl Module for SqliteModule {
    fn meta(&self) -> ModuleMeta {
        ("sqlite", "sqlite 数据").into()
    }

    fn on_init(&self, _app: Application) -> Result<(), ModuleError> {
        println!("sqlite 初始化");
        Ok(())
    }

    fn on_install(&self, _app: Application) -> Result<(), ModuleError> {
        Ok(())
    }

    fn on_uninstall(&self, _app: Application) -> Result<(), ModuleError> {
        Ok(())
    }

    fn close(&self) {
        println!("sqlite 关闭");
    }

    fn action(&self, _action: ModuleAction) -> ModuleActionResult {
        println!("执行成功:{:?}", _action);
        ModuleActionResult::success(None)
    }
    fn clone(&self) -> Box<dyn Module> {
        Box::new(Clone::clone(self))
    }
}
