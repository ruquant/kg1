use actix_web::{
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    App, Error, HttpServer,
};
use endpoints::service;
use kernel::DummyKernel;

mod endpoints;
mod host;
mod kernel;

fn app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    App::new().service(service::<DummyKernel>())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1";
    let port = 8080;

    HttpServer::new(app).bind((address, port))?.run().await
}
