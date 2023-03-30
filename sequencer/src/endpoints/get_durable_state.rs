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

/// Endpoint to get the durable state of a given key
pub async fn endpoint<D: Database + Send + 'static>(
    query: Query<Path>,
    seq: Data<Seq<D>>,
) -> impl Responder {
    let res = seq.as_ref().get_state(&query.path);
    match res {
        Some(data) => {
            let res = hex::encode(data);
            HttpResponseBuilder::new(StatusCode::OK).body(res)
        }
        None => HttpResponseBuilder::new(StatusCode::NOT_FOUND).finish(),
    }
}
