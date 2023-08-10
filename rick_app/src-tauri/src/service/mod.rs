mod test;
mod app;

use rick_core::error::{AppError, RickError};
use crate::app::application::Application;
use crate::global::RickResult;
use crate::service::test::init_test_service;

pub fn init_service(app: Application) -> Application{
    let register = app.service_register();
    app::init_service(&register);
    init_test_service(&register);
    app
}

pub fn common_result<T: RickError, S>(result: Result<usize, T>, val: S) -> RickResult<S> {
    match result {
        Ok(_) => {
            Ok(val)
        }
        Err(ref _err) => {
            Err(AppError::from(_err))
        }
    }
}

pub fn convert_result<T: RickError, S>(result: Result<S, T>) -> RickResult<S> {
    match result {
        Ok(_val) => {
            Ok(_val)
        }
        Err(ref _err) => {
            Err(AppError::from(_err))
        }
    }
}
