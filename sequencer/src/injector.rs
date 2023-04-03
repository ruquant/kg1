use reqwest::StatusCode;

pub struct Injector {
    client: reqwest::Client,
}

impl Injector {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn inject(&self, batch: Vec<Vec<u8>>) -> Result<(), ()> {
        if batch.is_empty() {
            return Ok(());
        }

        let batch = batch
            .iter()
            .map(|bytes| hex::encode(bytes))
            .collect::<Vec<String>>();

        let str = serde_json::to_string(&batch).unwrap();

        let res = self
            .client
            .post("http://127.0.0.1:8932/local/batcher/injection")
            .body(str)
            .send()
            .await;

        match res {
            Ok(res) => match res.status() {
                StatusCode::OK => Ok(()),
                _ => Err(()),
            },
            Err(_) => Err(()),
        }
    }
}
