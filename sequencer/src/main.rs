use actix_web::{
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    web::Data,
    App, Error, HttpServer,
};
use database::{sled::SledDatabase, Database};
use endpoints::service;
use kernel::DummyKernel;

mod database;
mod endpoints;
mod host;
mod kernel;

fn app<D: Database + 'static>(
    db: D,
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
        .service(service::<DummyKernel, SledDatabase>())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1";
    let port = 8080;
    let db_uri = "sequencer-storage";

    let db = SledDatabase::new(db_uri);

    HttpServer::new(move || app(db.clone()))
        .bind((address, port))?
        .run()
        .await
}