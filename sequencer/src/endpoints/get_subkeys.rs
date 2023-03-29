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

pub async fn endpoint<D: Database>(query: Query<Path>, db: Data<D>) -> impl Responder {
    let res = db.get_subkeys(&query.path);
    match res {
        Ok(data) => {
            let json = serde_json::to_string(&data);
            match json {
                Ok(json) => HttpResponseBuilder::new(StatusCode::OK).body(json),
                Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
            }
        }
        Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    }
}
