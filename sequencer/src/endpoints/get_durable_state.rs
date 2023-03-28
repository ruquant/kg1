use actix_web::{
    http::StatusCode,
    web::{Data, Query},
    HttpResponseBuilder, Responder,
};
use serde::{Deserialize, Serialize};

use crate::database::Database;

#[derive(Deserialize, Serialize)]
pub struct Path {
    path: String,
}

/// Endpoint to get the durable state of a given key
pub async fn endpoint<D: Database>(query: Query<Path>, db: Data<D>) -> impl Responder {
    let res = db.read(&query.path);
    match res {
        Ok(Some(data)) => {
            let res = hex::encode(data);
            HttpResponseBuilder::new(StatusCode::OK).body(res)
        }
        Ok(None) => HttpResponseBuilder::new(StatusCode::NOT_FOUND).finish(),
        Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    }
}
