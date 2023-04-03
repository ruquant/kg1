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
    use crate::{
        app,
        database::{sled::SledDatabase, Database},
        kernel::DummyKernel,
        node::Node,
    };
    use actix_web::{
        body::MessageBody,
        http::{Method, StatusCode},
        test,
    };
    use std::fs;

    #[derive(Clone)]
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

    impl Database for Db {
        fn write<'a>(
            &self,
            path: &str,
            data: &'a [u8],
        ) -> Result<&'a [u8], crate::database::DatabaseError> {
            self.inner.write(path, data)
        }

        fn read(&self, path: &str) -> Result<Option<Vec<u8>>, crate::database::DatabaseError> {
            self.inner.read(path)
        }

        fn get_subkeys(&self, path: &str) -> Result<Vec<String>, crate::database::DatabaseError> {
            self.inner.get_subkeys(path)
        }

        fn delete(&self, path: &str) -> Result<(), crate::database::DatabaseError> {
            self.inner.delete(path)
        }

        fn read_node(
            &self,
            path: &str,
        ) -> Result<Option<crate::database::Node>, crate::database::DatabaseError> {
            self.inner.read_node(path)
        }

        fn copy(&self, from: &str, to: &str) -> Result<(), crate::database::DatabaseError> {
            self.inner.copy(from, to)
        }
    }

    #[actix_web::test]
    async fn test_post_operation_content() {
        let db = Db::default();
        let node = Node::new::<DummyKernel>(db);

        let app = test::init_service(app(node)).await;
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
        let node = Node::new::<DummyKernel>(db);

        let app = test::init_service(app(node)).await;
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
        let node = Node::new::<DummyKernel>(db);

        let app = test::init_service(app(node)).await;

        let req = test::TestRequest::with_uri(&format!("/state?path=/foo"))
            .method(Method::GET)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_state_ok() {
        let db = Db::default();
        let node = Node::new::<DummyKernel>(db);

        let app = test::init_service(app(node)).await;

        let req = test::TestRequest::with_uri("/operations")
            .method(Method::POST)
            .set_json(Body {
                data: "8801010101".to_string(),
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
        let node = Node::new::<DummyKernel>(db);

        let app = test::init_service(app(node)).await;

        let req = test::TestRequest::with_uri("/operations")
            .method(Method::POST)
            .set_json(Body {
                data: "8801010101".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let req = test::TestRequest::with_uri("/operations")
            .method(Method::POST)
            .set_json(Body {
                data: "8801010101".to_string(),
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
    async fn cannot_get_slash() {
        let db = Db::default();
        let node = Node::new::<DummyKernel>(db);

        let app = test::init_service(app(node)).await;

        let req = test::TestRequest::with_uri("/subkeys?path=/")
            .method(Method::GET)
            .to_request();
        let resp = test::call_service(&app, req).await;

        println!("{}", resp.status());

        assert!(resp.status() == StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[actix_web::test]
    async fn test_get_one_subkey_empty() {
        let db = Db::default();
        let node = Node::new::<DummyKernel>(db);

        let app = test::init_service(app(node)).await;

        let req = test::TestRequest::with_uri("/operations")
            .method(Method::POST)
            .set_json(Body {
                data: "01010101".to_string(),
            })
            .to_request();
        let _ = test::call_service(&app, req).await;

        let req = test::TestRequest::with_uri("/subkeys?path=/counter")
            .method(Method::GET)
            .to_request();
        let resp = test::call_service(&app, req).await;

        let body = resp.into_body().try_into_bytes().unwrap().to_vec();
        let response: Vec<String> =
            serde_json::from_str(&String::from_utf8(body).unwrap()).unwrap();
        let expeted: Vec<String> = vec![];

        assert_eq!(response, expeted);
    }
}
