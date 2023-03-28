use actix_web::{
    http::StatusCode,
    web::{self, Query},
    HttpResponseBuilder, Responder, Scope,
};
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

async fn post_operation(body: web::Json<Body>, db: web::Data<sled::Db>) -> impl Responder {
    // Check the body
    let data = hex::decode(&body.data).unwrap();
    let db = db.as_ref().clone();

    println!("Operation has been submitted to the sequencer");

    let mut host = NativeHost::new(db);

    host.add_input(data);

    DummyKernel::entry(&mut host);
    "Operation submitted"
}

#[derive(Deserialize, Serialize)]
pub struct Path {
    path: String,
}

async fn get_durable_state(query: Query<Path>, db: web::Data<sled::Db>) -> impl Responder {
    let res = db.get(&query.path);
    match res {
        Ok(Some(data)) => {
            let res = hex::encode(data);
            HttpResponseBuilder::new(StatusCode::OK).body(res)
        }
        Ok(None) => HttpResponseBuilder::new(StatusCode::NOT_FOUND).finish(),
        Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    }
}

/// Exposes all the endpoint of the application
pub fn service<K: Kernel>() -> Scope {
    web::scope("")
        .route("/operations", web::post().to(post_operation))
        .route("/state", web::get().to(get_durable_state))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{app, endpoints::Body};
    use actix_web::{
        body::MessageBody,
        http::{Method, StatusCode},
        test,
    };

    struct Db {
        inner: sled::Db,
        path: String,
    }

    impl Default for Db {
        fn default() -> Self {
            let path: String = format!("/tmp/{}", uuid::Uuid::new_v4().to_string());
            let db = sled::open(&path).unwrap();
            Self { inner: db, path }
        }
    }

    impl Drop for Db {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[actix_web::test]
    async fn test_post_operation_content() {
        let db = Db::default();

        let app = test::init_service(app(db.inner.clone())).await;
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
        let db = Db::default();

        let app = test::init_service(app(db.inner.clone())).await;
        let req = test::TestRequest::with_uri("/operations")
            .method(Method::POST)
            .set_json(Body {
                data: "01010101".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_state_404() {
        let db = Db::default();

        let app = test::init_service(app(db.inner.clone())).await;

        let req = test::TestRequest::with_uri(&format!("/state?path=/foo"))
            .method(Method::GET)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_state_ok() {
        let db = Db::default();

        let app = test::init_service(app(db.inner.clone())).await;

        let req = test::TestRequest::with_uri("/operations")
            .method(Method::POST)
            .set_json(Body {
                data: "01010101".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let req = test::TestRequest::with_uri("/state?path=/counter")
            .method(Method::GET)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_state_value() {
        let db = Db::default();

        let app = test::init_service(app(db.inner.clone())).await;

        let req = test::TestRequest::with_uri("/operations")
            .method(Method::POST)
            .set_json(Body {
                data: "01010101".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let req = test::TestRequest::with_uri("/state?path=/counter")
            .method(Method::GET)
            .to_request();

        let resp = test::call_service(&app, req).await;
        let body = resp.into_body().try_into_bytes().unwrap().to_vec();
        let str = String::from_utf8(body).unwrap();
        assert_eq!("0000000000000001", str);
    }
}
