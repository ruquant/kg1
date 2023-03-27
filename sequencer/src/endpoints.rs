use actix_web::{get, web, Responder, Scope};

#[get("")]
async fn index() -> impl Responder {
    "Hello world!"
}

pub fn service() -> Scope {
    web::scope("/").service(index)
}
