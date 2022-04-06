pub mod json;
pub mod ytm_utils;

pub use json::*;
pub use ytm_utils::*;

use reqwest::Client;
use std::{fs, sync::Arc};
use ytm_utils::utils::{Endpoint, Headers};
use tokio::sync::Mutex;

pub struct YtMusicClient {
    client: Arc<Client>,
    headers: Arc<Mutex<Headers>>,
}

impl YtMusicClient {
    pub fn new(headers_file: &str) -> YtMusicClient {
        YtMusicClient {
            client: Arc::new(Client::new()),
            headers: Arc::new(Mutex::new(Headers::new(
                &fs::read_to_string(headers_file).expect("Unable to read headers file"),
            ))),
        }
    }

    pub async fn set_headers(&self) {
        let headers = self.headers.clone();
        let mut headers = headers.lock().await;
        headers.set_visitor_id(&self.client).await;
        headers.set_authorization();
    }

    pub fn endpoint(&self, endpoint: &str) -> Endpoint {
        Endpoint::new(endpoint, self.client.clone(), self.headers.clone())
    }
}

#[cfg(test)]
mod tests;
