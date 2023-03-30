use std::{collections::VecDeque, marker::PhantomData};

use tezos_smart_rollup_host::input::Message;
use tokio::sync::{
    mpsc::{self, Sender},
    oneshot,
};

use crate::{
    database::Database,
    host::{Host, NativeHost},
    kernel::Kernel,
};

//// Sequencer
/// Or Low Latency

trait LowLatency {
    fn on_operation(&mut self, op: Vec<u8>);
}

struct Sequencer<H, K, D>
where
    H: Host<D>,
    K: Kernel,
    D: Database,
{
    batch: VecDeque<Message>,
    tezos_level: u32,
    host: H,
    kernel_marker: PhantomData<K>,
    database_marker: PhantomData<D>,
}

impl<H, K, D> LowLatency for Sequencer<H, K, D>
where
    H: Host<D>,
    K: Kernel,
    D: Database,
{
    fn on_operation(&mut self, payload: Vec<u8>) {
        // Create the message
        let msg_index = self.batch.len().try_into().unwrap(); // Handle this error
        let tezos_level = self.tezos_level;
        let msg_1 = Message::new(tezos_level, msg_index, payload.clone());
        let msg_2 = Message::new(tezos_level, msg_index, payload); // Exactly the same message, but I cannot clone it

        // Add the message to the batch
        self.batch.push_back(msg_1);

        // Add the message to the runtime
        self.host.add_message(msg_2);

        // Execute the kernel with the host
        K::entry(&mut self.host);
    }
}

//// The queue thing....

pub struct Seq<D>
where
    D: Database,
{
    db: D,
    tx: Sender<QueueMsg>,
}
enum QueueContent {
    Message(Vec<u8>),
}

struct QueueMsg {
    promise: oneshot::Sender<()>,
    content: QueueContent,
}

impl<D> Seq<D>
where
    D: Database + Send + 'static,
{
    pub fn new<K>(db: D) -> Self
    where
        K: Kernel + Send,
    {
        let (tx, mut rx) = mpsc::channel::<QueueMsg>(1024);

        let db1 = db.clone();

        tokio::spawn(async move {
            let mut running = true;

            let mut sequencer = Sequencer::<NativeHost<D>, K, D> {
                batch: VecDeque::default(),
                tezos_level: 0,
                host: NativeHost::new(db1),
                kernel_marker: Default::default(),
                database_marker: Default::default(),
            };

            while running {
                match rx.recv().await {
                    None => running = false,
                    Some(msg) => {
                        let QueueMsg { promise, content } = msg;
                        let _ = promise.send(());
                        match content {
                            QueueContent::Message(msg) => sequencer.on_operation(msg),
                        }
                    }
                }
            }
        });

        Self { db, tx }
    }

    pub fn get_state(&self, path: &str) -> Option<Vec<u8>> {
        match self.db.read(path) {
            Ok(Some(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_subkeys(&self, path: &str) -> Option<Vec<String>> {
        match self.db.get_subkeys(path) {
            Ok(value) => Some(value),
            _ => None,
        }
    }

    pub async fn add_operation(&self, operation: Vec<u8>) {
        let (tx, rx) = oneshot::channel::<()>();
        let msg = QueueMsg {
            promise: tx,
            content: QueueContent::Message(operation),
        };
        let _ = self.tx.send(msg).await;
        let _ = rx.await;
        println!("Message submitted")
    }
}
