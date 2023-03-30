use actix_web::{
    http::StatusCode,
    web::{Data, Query},
    HttpResponseBuilder, Responder,
};
use serde::{Deserialize, Serialize};

use crate::{database::Database, sequencer::Seq};

#[derive(Deserialize, Serialize)]
pub struct Path {
    path: String,
}

pub async fn endpoint<D: Database + Send + 'static>(
    query: Query<Path>,
    seq: Data<Seq<D>>,
) -> impl Responder {
    let res = seq.as_ref().get_subkeys(&query.path);
    match res {
        Some(data) => {
            let json = serde_json::to_string(&data);
            match json {
                Ok(json) => HttpResponseBuilder::new(StatusCode::OK).body(json),
                Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
            }
        }
        None => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    }
}
