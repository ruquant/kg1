use crate::{database::Database, node::Node};
use actix_web::{
    web::{Data, Json},
    Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Body {
    pub data: String,
}

/// Endpoint to dubmit a L2 operation to the sequencer
pub async fn endpoint<D: Database + Send + 'static>(
    body: Json<Body>,
    seq: Data<Node<D>>,
) -> impl Responder {
    // Check the body
    let data = hex::decode(&body.data).unwrap();

    seq.as_ref().add_operation(data).await;

    "Operation submitted"
}
