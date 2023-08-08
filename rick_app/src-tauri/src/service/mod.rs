mod test;
mod app;

use crate::app::application::Application;
use crate::service::test::init_test_service;

pub fn init_service(app: Application) -> Application{
    let register = app.service_register();
    app::init_service(&register);
    init_test_service(&register);
    app
}