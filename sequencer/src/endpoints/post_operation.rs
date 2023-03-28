use actix_web::{
    web::{Data, Json},
    Responder,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::Database,
    host::{AddInput, NativeHost},
    kernel::{DummyKernel, Kernel},
};

#[derive(Deserialize, Serialize)]
pub struct Body {
    pub data: String,
}

/// Endpoint to dubmit a L2 operation to the sequencer
pub async fn endpoint<D: Database>(body: Json<Body>, db: Data<D>) -> impl Responder {
    // Check the body
    let data = hex::decode(&body.data).unwrap();
    let db = db.as_ref().clone();

    println!("Operation has been submitted to the sequencer");

    let mut host = NativeHost::<D>::new(db);

    host.add_input(data);

    DummyKernel::entry(&mut host);
    "Operation submitted"
}
