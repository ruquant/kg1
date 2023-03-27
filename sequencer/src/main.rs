use actix_web::{
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    App, Error, HttpServer,
};
use endpoints::service;

mod endpoints;

fn app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    App::new().service(service())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1";
    let port = 8080;

    HttpServer::new(app).bind((address, port))?.run().await
}

#[cfg(test)]
mod tests {
    use crate::app;
    use actix_web::{body::MessageBody, http::StatusCode, test};

    #[actix_web::test]
    async fn test_hello_world_content() {
        let app = test::init_service(app()).await;
        let req = test::TestRequest::default().to_request();
        let resp = test::call_service(&app, req).await;

        let body = resp.into_body().try_into_bytes().unwrap().to_vec();
        let str = String::from_utf8(body).unwrap();

        assert_eq!(str, "Hello world!")
    }

    #[actix_web::test]
    async fn test_hello_world_status() {
        let app = test::init_service(app()).await;
        let req = test::TestRequest::default().to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
