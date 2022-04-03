pub mod json;
pub mod ytm_utils;

pub use json::*;
pub use ytm_utils::*;

use reqwest::Client;
use std::{cell::RefCell, fs, rc::Rc};
use ytm_utils::utils::{Endpoint, Headers};

pub struct YtMusicClient {
    client: Rc<Client>,
    headers: Rc<RefCell<Headers>>,
}

impl YtMusicClient {
    pub fn new(headers_file: &str) -> YtMusicClient {
        YtMusicClient {
            client: Rc::new(Client::new()),
            headers: Rc::new(RefCell::new(Headers::new(
                &fs::read_to_string(headers_file).expect("Unable to read headers file"),
            ))),
        }
    }

    pub async fn set_headers(&self) {
        self.headers.borrow_mut().set_visitor_id(&self.client).await;
        self.headers.borrow_mut().set_authorization();
    }

    pub fn endpoint(&self, endpoint: &str) -> Endpoint {
        Endpoint::new(endpoint, self.client.clone(), self.headers.clone())
    }
}
