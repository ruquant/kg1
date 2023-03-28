use actix_web::{web, Responder, Scope};
use serde::{Deserialize, Serialize};

use crate::{
    host::{AddInput, NativeHost},
    kernel::DummyKernel,
    kernel::Kernel,
};

#[derive(Deserialize, Serialize)]
struct Body {
    pub data: String,
}

async fn post_operation(body: web::Json<Body>) -> impl Responder {
    // Check the body
    let data = hex::decode(&body.data).unwrap();

    println!("Operation has been submitted to the sequencer");

    let mut host = NativeHost::default();
    host.add_input(data);

    DummyKernel::entry(&mut host);
    "Operation submitted"
}

/// Exposes all the endpoint of the application
pub fn service<K: Kernel>() -> Scope {
    web::scope("").route("/operations", web::post().to(post_operation))
}

#[cfg(test)]
mod tests {
    use crate::{app, endpoints::Body};
    use actix_web::{
        body::MessageBody,
        http::{Method, StatusCode},
        test,
    };

    #[actix_web::test]
    async fn test_post_operation_content() {
        let app = test::init_service(app()).await;
        let req = test::TestRequest::with_uri("/operations")
            .method(Method::POST)
            .set_json(Body {
                data: "00010101".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;

        let body = resp.into_body().try_into_bytes().unwrap().to_vec();
        let str = String::from_utf8(body).unwrap();

        assert_eq!(str, "Operation submitted")
    }

    #[actix_web::test]
    async fn test_post_operation_status() {
        let app = test::init_service(app()).await;
        let req = test::TestRequest::with_uri("/operations")
            .method(Method::POST)
            .set_json(Body {
                data: "01010101".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
