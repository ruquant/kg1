use serde::Deserialize;
use tokio::sync::{
    mpsc::{self, Sender},
    oneshot,
};

use crate::{
    database::Database,
    host::{Host, NativeHost},
    kernel::Kernel,
    low_latency::LowLatency,
    sequencer::Sequencer,
    tezos_listener::TezosListener,
};

//// The queue thing....

#[derive(Clone)]
pub struct Node<D>
where
    D: Database,
{
    database: D,
    tx: Sender<QueueMsg>,
}

#[derive(Deserialize)]
pub struct TezosHeader {
    pub hash: String,
    pub level: u64,
    pub predecessor: String,
}

pub enum QueueContent {
    Message(Vec<u8>),
    TezosHeader(TezosHeader),
}

pub struct QueueMsg {
    pub promise: Option<oneshot::Sender<()>>,
    pub content: QueueContent,
}

impl<D> Node<D>
where
    D: Database + Send + 'static,
{
    pub fn new<K>(db: D) -> Self
    where
        K: Kernel + Send + 'static,
    {
        let (tx, mut rx) = mpsc::channel::<QueueMsg>(1024);

        let database = db.clone();

        let mut sequencer = Sequencer::new();
        let host = NativeHost::new(db);
        let mut low_latency = LowLatency::<K, NativeHost<D>, D>::new(host);

        tokio::spawn(async move {
            let mut running = true;

            while running {
                match rx.recv().await {
                    None => running = false,
                    Some(msg) => {
                        let QueueMsg { promise, content } = msg;
                        if let Some(promise) = promise {
                            let _ = promise.send(());
                        }

                        match content {
                            QueueContent::Message(msg) => {
                                Node::on_operation(&mut sequencer, &mut low_latency, msg)
                            }
                            QueueContent::TezosHeader(_) => println!("yolo"),
                        }
                    }
                }
            }
        });

        TezosListener::listen(tx.clone());

        Self { database, tx }
    }

    /// Process an operation
    fn on_operation<K, H>(
        sequencer: &mut Sequencer,
        low_latency: &mut LowLatency<K, H, D>,
        msg: Vec<u8>,
    ) where
        K: Kernel,
        H: Host<D>,
    {
        let msg = sequencer.on_operation(msg);
        low_latency.on_operation(msg);
    }

    /// Get the state of a given key
    pub fn get_state(&self, path: &str) -> Option<Vec<u8>> {
        match self.database.read(path) {
            Ok(Some(value)) => Some(value),
            _ => None,
        }
    }

    /// Get the subkeys of a given keys
    pub fn get_subkeys(&self, path: &str) -> Option<Vec<String>> {
        match self.database.get_subkeys(path) {
            Ok(value) => Some(value),
            _ => None,
        }
    }

    /// Add an operation to the sequencer and simulate it
    pub async fn add_operation(&self, operation: Vec<u8>) {
        let (tx, rx) = oneshot::channel::<()>();
        let msg = QueueMsg {
            promise: Some(tx),
            content: QueueContent::Message(operation),
        };
        let _ = self.tx.send(msg).await;
        let _ = rx.await;
        println!("Message submitted")
    }
}
