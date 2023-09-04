use std::{fs, vec};
use std::fs::File;
use std::sync::Arc;
use image::math;

use serde::{Deserialize, Serialize};
use tauri::api::dialog::FileDialogBuilder;
use rick_core::error::AppError;
use crate::app::service::{ClosureFnService, ServiceInvoke, ServiceRegister, ServiceResult};
use crate::global::RickResult;
use crate::service::app::get_logo_dir;
use crate::utils::ThreadSignal;

/// 服务注册
pub fn init_service(_register: &ServiceRegister) {
    _register.register("/common/file/read/open".into(), ClosureFnService::new(file_read_open));
    _register.register("/common/file/save/open".into(), ClosureFnService::new(file_save_open));
    _register.register("/common/dir/open".into(), ClosureFnService::new(dir_open));
    _register.register_closure_fn("/common/logo/raed", logo_read);
    _register.register_closure_fn("/common/file/read", file_read);
}

/// 文件过滤
#[derive(Serialize, Deserialize, Debug)]
pub struct FileFilter {
    /// 文件名称
    name: String,
    /// 类型
    extensions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileOpen {
    /// 文件过滤
    filter: Option<Vec<FileFilter>>,
    /// 标题
    title: Option<String>,
    /// 默认名称
    default_name: Option<String>,
    /// 打开默认的目录
    default_dir: Option<String>,
    /// 是否支持多选
    multiple: bool,
    /// 是否需要文件内容信息
    need_content: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileOpenResult {
    /// 文件路径
    path: String,
    /// 文件内容
    content: Option<Vec<u8>>,
    /// 内容状态
    content_status: Option<bool>,
    /// 原因
    fail_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRead {
    path: String
}

/// 读取Logo 数据
fn logo_read(read: FileRead) -> RickResult<Vec<u8>> {
    let mut logo_dir = get_logo_dir();
    logo_dir.push(read.path);
    match fs::read(logo_dir) {
        Ok(_data) => Ok(_data),
        Err(ref _err) => {
            Err(AppError::new(500, _err.to_string()))
        }
    }
}

/// 读取Logo 数据
fn file_read(read: FileRead) -> RickResult<Vec<u8>> {

    match fs::read(read.path) {
        Ok(_data) => {
            println!("文件读取成功:{:#?}", _data.len());
            Ok(_data)
        },
        Err(ref _err) => {
            println!("文件读取失败:{:#?}", _err);
            Err(AppError::new(500, _err.to_string()))
        }
    }
}


fn file_save_open(service: ServiceInvoke) {
    common_file_dialog(service, |builder, service, file_open, notify| {
        builder.save_file(move |file| {
            if let None = file {
                println!("没选择文件");
                service.reject(ServiceResult::<i32>::fail_reason("请选择文件"));
                notify.notify_all();
                return;
            }
            let _file_path: String = file.unwrap().to_str().unwrap().into();
            service.resolve(ServiceResult::success_data(vec![FileOpenResult {
                path: _file_path,
                content: None,
                fail_reason: None,
                content_status: None,
            }]));
            notify.notify_all();
        });
    });
}

fn file_read_open(service: ServiceInvoke) {
    common_file_dialog(service, |builder, service, file_open, notify| {
        if file_open.multiple {
            builder.pick_files(move |files| {
                if let None = files {
                    println!("没选择文件");
                    service.reject(ServiceResult::<i32>::fail_reason("请选择文件"));
                    notify.notify_all();
                    return;
                }
                let mut _open_result_vec: Vec<FileOpenResult> = Vec::new();
                let _need_content = file_open.need_content;
                for _file in files.unwrap() {
                    let _file_path: String = _file.to_str().unwrap().into();
                    _open_result_vec.push(file_read_result_process(_need_content, _file_path));
                }
                service.resolve(ServiceResult::success_data(_open_result_vec));
                notify.notify_all();
            });
        } else {
            builder.pick_file(move |file| {
                if let None = file {
                    println!("没选择文件");
                    service.reject(ServiceResult::<i32>::fail_reason("请选择文件"));
                    notify.notify_all();
                    return;
                }
                let _file_path: String = file.unwrap().to_str().unwrap().into();
                let _need_content = file_open.need_content;
                service.resolve(ServiceResult::success_data(vec![file_read_result_process(_need_content, _file_path)]));
                notify.notify_all();
            });
        }
    });
}

fn dir_open(service: ServiceInvoke) {
    common_file_dialog(service, |builder, service, file_open, notify| {
        // 是否多选
        if file_open.multiple {
            builder.pick_folders(move |files| {
                if let None = files {
                    println!("没选择目录");
                    service.reject(ServiceResult::<i32>::fail_reason("请选择文件目录"));
                    notify.notify_all();
                    return;
                }
                let mut _open_result_vec: Vec<FileOpenResult> = Vec::new();
                let _need_content = file_open.need_content;
                for _file in files.unwrap() {
                    let _file_path: String = _file.to_str().unwrap().into();
                    _open_result_vec.push(FileOpenResult {
                        path: _file_path,
                        content: None,
                        content_status: None,
                        fail_reason: None,
                    });
                }
                service.resolve(ServiceResult::success_data(_open_result_vec));
                notify.notify_all();
            });
        } else {
            builder.pick_folder(move |file| {
                if let None = file {
                    service.reject(ServiceResult::<i32>::fail_reason("请选择文件目录"));
                    notify.notify_all();
                    return;
                }
                let _file_path: String = file.unwrap().to_str().unwrap().into();
                service.resolve(ServiceResult::success_data(vec![FileOpenResult {
                    path: _file_path,
                    content: None,
                    content_status: None,
                    fail_reason: None,
                }]));
                notify.notify_all();
            });
        }
    });
}

/// ======================================== [ 私有方法 ] =========================================

fn file_read_result_process(_need_content: bool, _file_path: String) -> FileOpenResult {
    if !_need_content {
        return FileOpenResult {
            path: _file_path,
            content: None,
            fail_reason: None,
            content_status: None,
        };
    } else {
        let content = fs::read(&_file_path);
        return match content {
            Ok(_data) => {
                FileOpenResult {
                    path: _file_path,
                    content: Some(_data),
                    content_status: Some(true),
                    fail_reason: None,
                }
            }
            Err(_err) => {
                FileOpenResult {
                    path: _file_path,
                    content: Some(vec![]),
                    content_status: Some(false),
                    fail_reason: Some(_err.to_string()),
                }
            }
        };
    }
}

/// 通用文件对话框
fn common_file_dialog<F: FnOnce(FileDialogBuilder, ServiceInvoke, FileOpen, Arc<ThreadSignal>)>(service: ServiceInvoke, callback: F) {
    let file_open = service.get::<FileOpen>();
    if let Err(_err) = file_open {
        service.reject(ServiceResult::<i32>::fail_reason("请求错误"));
        return;
    }
    let file_open = file_open.unwrap();
    let wait = ThreadSignal::new_arc();
    let notify = wait.clone();
    let builder = FileDialogBuilder::new();
    callback(set_builder(builder, &file_open), service, file_open, notify);
    wait.wait();
}

fn set_builder(mut builder: FileDialogBuilder, file_open: &FileOpen) -> FileDialogBuilder {
    if let Some(ref _title) = file_open.title {
        builder = builder.set_title(_title.as_str());
    }
    if let Some(ref _current_dir) = file_open.default_dir {
        builder = builder.set_directory(_current_dir);
    }
    if let Some(ref _default_name) = file_open.default_name {
        builder = builder.set_file_name(_default_name);
    }
    if let Some(ref _filters) = file_open.filter {
        for _filter in _filters {
            let _extensions_refs: Vec<&str> = _filter.extensions.iter().map(|s| s.as_str()).collect();
            builder = builder.add_filter(_filter.name.as_str(), _extensions_refs.as_slice());
        }
    } else {
        builder = builder.add_filter("*", &["*"]);
    }
    builder
}