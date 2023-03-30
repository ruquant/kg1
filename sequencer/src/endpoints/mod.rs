use actix_web::{web, Scope};

use crate::database::Database;

mod get_durable_state;
mod get_subkeys;
mod post_operation;

/// Exposes all the endpoint of the application
pub fn service<D>() -> Scope
where
    D: Database + Send + 'static,
{
    web::scope("")
        .route("/operations", web::post().to(post_operation::endpoint::<D>))
        .route("/state", web::get().to(get_durable_state::endpoint::<D>))
        .route("/subkeys", web::get().to(get_subkeys::endpoint::<D>))
}

#[cfg(test)]
mod tests {
    use super::post_operation::Body;
    use crate::{app, database::sled::SledDatabase};
    use actix_web::{
        body::MessageBody,
        http::{Method, StatusCode},
        test,
    };
    use std::fs;

    struct Db {
        inner: SledDatabase,
        path: String,
    }

    impl Default for Db {
        fn default() -> Self {
            let path: String = format!("/tmp/{}", uuid::Uuid::new_v4().to_string());
            let db = SledDatabase::new(&path);
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
        assert_eq!("0000000000000002", str);
    }

    #[actix_web::test]
    async fn test_get_subkeys_empty() {
        let db = Db::default();

        let app = test::init_service(app(db.inner.clone())).await;
        let req = test::TestRequest::with_uri("/subkeys?path=/")
            .method(Method::GET)
            .to_request();

        let resp = test::call_service(&app, req).await;

        let body = resp.into_body().try_into_bytes().unwrap().to_vec();
        let response: Vec<String> =
            serde_json::from_str(&String::from_utf8(body).unwrap()).unwrap();
        let expeted: Vec<String> = Vec::default();

        assert_eq!(response, expeted);
    }

    #[actix_web::test]
    async fn test_get_one_subkey_empty() {
        let db = Db::default();

        let app = test::init_service(app(db.inner.clone())).await;

        let req = test::TestRequest::with_uri("/operations")
            .method(Method::POST)
            .set_json(Body {
                data: "01010101".to_string(),
            })
            .to_request();
        let _ = test::call_service(&app, req).await;

        let req = test::TestRequest::with_uri("/subkeys?path=/")
            .method(Method::GET)
            .to_request();
        let resp = test::call_service(&app, req).await;

        let body = resp.into_body().try_into_bytes().unwrap().to_vec();
        let response: Vec<String> =
            serde_json::from_str(&String::from_utf8(body).unwrap()).unwrap();
        let expeted: Vec<String> = vec!["counter".to_string()];

        assert_eq!(response, expeted);
    }
}
