use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, RwLock};
use crate::app::application::{Application, ApplicationEvent};
use crate::app::module::{Module, ModuleError};

#[derive(Debug)]
pub enum ModuleInfoStateEnum {
    /// 初始化
    INIT,
    /// 已加载
    INSTALL
}

pub struct ModuleInfo {
    /// 模块
    module: Box<dyn Module>,
    /// 是否 install
    state: ModuleInfoStateEnum,
}

impl Display for ModuleInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let meta = self.module.meta();
        f.write_fmt(format_args!("Module: {} Desc: {} State: {:?}", meta.name, meta.desc, self.state))
    }
}

impl ModuleInfo {
    pub fn new(module: Box<dyn Module>) -> Self {
        Self {
            module,
            state: ModuleInfoStateEnum::INIT,
        }
    }
}

/// 模块管理器
#[derive(Clone)]
pub struct ModuleManager {
    _module_map: Arc<RwLock<HashMap<&'static str, ModuleInfo>>>,
}

#[allow(unused)]
impl ModuleManager {
    /// 创建新的模块管理器
    pub fn new() -> Self {
        Self {
            _module_map: Arc::new(RwLock::new(HashMap::new()))
        }
    }
}
unsafe impl Send for ModuleManager{}
#[allow(unused)]
impl ModuleManager {
    /// 添加模块
    pub fn add_module(&self, module: Box<dyn Module>) {
        let mut map = self._module_map.write().unwrap();
        map.entry(module.meta().name).or_insert(ModuleInfo::new(module));
    }
    /// 添加多个模块
    pub fn add_module_iter<I: Iterator<Item=Box<dyn Module>>>(&self, iter: I) {
        let mut map = self._module_map.write().unwrap();
        iter.map(|module| {
            map.entry(module.meta().name).or_insert(ModuleInfo::new(module));
        });
    }

    pub fn get_module(&self, module_name: &str) -> Option<Box<dyn Module>>{
        let _map = self._module_map.read().unwrap();
        if let Some(_module_info) = _map.get(module_name) {
            let _module = _module_info.module.clone();
            return Some(_module);
        }
        None
    }

    pub(crate) fn on_init(&self, app: Application) -> Result<(), ModuleError> {
        let start = self.clone();
        let stop = self.clone();
        app.event_context().on_into(ApplicationEvent::Started, move |_app: &Application| {
            start.on_install(_app.clone());
        });
        app.event_context().on_into(ApplicationEvent::Stoped, move |_app: &Application| {
            stop.close();
        });

        for item in self._module_map.read().unwrap().values() {
            item.module.on_init(app.clone());
        }
        Ok(())
    }

    fn on_install(&self, app: Application) -> Result<(), ModuleError> {
        for item in self._module_map.read().unwrap().values() {
            item.module.on_init(app.clone());
        }
        Ok(())
    }
    fn close(&self) {
        for item in self._module_map.read().unwrap().values() {
            item.module.close();
        }
    }
}