use actix_web::{
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    web::Data,
    App, Error, HttpServer,
};
use endpoints::service;
use kernel::DummyKernel;

mod endpoints;
mod host;
mod kernel;

fn app(
    db: sled::Db,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    let db_state = Data::new(db);

    App::new()
        .app_data(db_state)
        .service(service::<DummyKernel>())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1";
    let port = 8080;
    let db_uri = "sequencer-storage";

    let db: sled::Db = sled::open(db_uri).unwrap();

    HttpServer::new(move || app(db.clone()))
        .bind((address, port))?
        .run()
        .await
}
