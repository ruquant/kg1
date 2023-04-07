use crate::node::{QueueContent, QueueMsg, TezosHeader};
use futures_util::StreamExt;
use tokio::sync::mpsc::Sender;

pub struct TezosListener {}

impl TezosListener {
    pub fn listen(sender: Sender<QueueMsg>) {
        tokio::spawn(async move {
            let mut stream = reqwest::get("http://localhost:18731/monitor/heads/main")
                .await
                .unwrap()
                .bytes_stream();

            while let Some(Ok(item)) = stream.next().await {
                let bytes = item.to_vec();

                let header = serde_json::from_slice::<TezosHeader>(&bytes);
                match header {
                    Err(_) => {}
                    Ok(header) => {
                        let msg = QueueMsg {
                            promise: None,
                            content: QueueContent::TezosHeader(header),
                        };

                        let _ = sender.send(msg).await;
                    }
                }
            }
        });
    }
}
