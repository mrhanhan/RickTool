mod model;

use serde::{Deserialize, Serialize};
use rick_core::error::AppError;
use crate::app::service::{ServiceInvoke, ServiceRegister};
use crate::global::RickResult;
use crate::service::terminal::model::SimpleTerminal;
use crate::store::StrId;

pub fn init_service(_register: &ServiceRegister) {

    _register.register_run_fn("/terminal/list", list_terminal);
    _register.register_invoke_fn("/terminal/create", create_terminal);
    _register.register_closure_fn("/terminal/kill", kill_terminal);
    _register.register_closure_fn("/terminal/read", read_terminal);
    _register.register_closure_fn("/terminal/write", write_terminal);
}

static mut TERMINAL_VEC: Vec<SimpleTerminal> = Vec::new();



/// ====================================== [model] ===========================================

#[derive(Serialize, Deserialize)]
struct CreateTerminalModel{
    id: String,
    name: String
}
#[derive(Serialize, Deserialize)]
struct ReadTerminalModel{
    id: String,
    offset: usize
}
#[derive(Serialize, Deserialize)]
struct WriteTerminalModel{
    id: String,
    data: Vec<u8>
}


/// ====================================== [api] ===========================================

fn list_terminal() -> RickResult<Vec<SimpleTerminal>>{
    Ok(unsafe {TERMINAL_VEC.clone()})
}

fn create_terminal(invoke: &ServiceInvoke) -> RickResult<()> {
    let value = invoke.data();
    let id: String = value.get("id").unwrap().as_str().unwrap().into();
    let name: String = value.get("name").unwrap().as_str().unwrap().into();
    let terminal = SimpleTerminal::new(id, name, invoke.window());
    terminal.start();
    unsafe {TERMINAL_VEC.push(terminal)};
    return Ok(());
}

fn read_terminal(model: ReadTerminalModel) -> RickResult<Vec<u8>> {
    let index = find_by_id(&model.id);
    if let Some(_index) = index {
        let terminal = unsafe {TERMINAL_VEC.get(_index)};
        if let Some(_terminal) = terminal {
            return Ok(_terminal.read(model.offset));
        }
    }
    Err(AppError::new(404, "终端不存在"))
}

fn write_terminal(model: WriteTerminalModel) -> RickResult<()> {
    let index = find_by_id(&model.id);
    if let Some(_index) = index {
        let terminal = unsafe {TERMINAL_VEC.get(_index)};
        if let Some(_terminal) = terminal {
            _terminal.write(model.data.as_slice());
            return Ok(());
        }
    }
    Err(AppError::new(404, "终端不存在"))
}

fn kill_terminal(id: StrId) -> RickResult<()> {
    let index = find_by_id(&id.id);
    if let Some(_index) = index {
        let terminal = unsafe {TERMINAL_VEC.remove(_index)};
        terminal.kill();
    }
    Ok(())
}

fn find_by_id(id: &String) -> Option<usize>{
    let data = unsafe {&TERMINAL_VEC};
    let mut index = 0_usize;
    for x in data {
        if x.id.eq(id) {
            return Some(index);
        }
        index = index + 1;
    }
    return None;
}

