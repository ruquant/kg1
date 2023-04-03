use actix_web::{
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    web::Data,
    App, Error, HttpServer,
};
use database::{sled::SledDatabase, Database};
use endpoints::service;
use kernel::DummyKernel;
use node::Node;

mod database;
mod endpoints;
mod host;
mod kernel;
mod low_latency;
mod node;
mod sequencer;
mod tezos_listener;

fn app<D: Database + Send + 'static>(
    node: Node<D>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    let state = Data::new(node);

    App::new().app_data(state).service(service::<D>())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1";
    let port = 8080;
    let db_uri = "sequencer-storage";

    let db = SledDatabase::new(db_uri);

    let node = Node::new::<DummyKernel>(db);

    HttpServer::new(move || app(node.clone()))
        .bind((address, port))?
        .run()
        .await
}
