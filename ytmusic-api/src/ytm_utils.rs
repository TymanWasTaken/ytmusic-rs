pub mod utils {
    use regex::Regex;
    use reqwest::header::{HeaderMap, HeaderName};
    use reqwest::{Body, Client, Method, RequestBuilder, Response};
    use serde_json::Value;
    use sha1::{Digest, Sha1};
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::str::FromStr;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug)]
    pub struct Headers {
        x_goog_visitor_id: Option<String>,
        authorization: Option<String>,
        sapisid: String,
        headers_text: String,
    }

    impl Headers {
        pub fn new(data: &String) -> Headers {
            let cookie_line = data
                .split("\n")
                .find(|&x| x.starts_with("Cookie: "))
                .expect("Fuck v2")
                .replace("Cookie: ", "");
            let sapisid = cookie_line
                .split(";")
                .find(|&x| x.contains("__Secure-3PAPISID="))
                .expect("Fuck v3")
                .trim()
                .split("=")
                .nth(1)
                .expect("Fuck v4");

            Headers {
                x_goog_visitor_id: None,
                authorization: None,
                sapisid: sapisid.to_string(),
                headers_text: data.to_string(),
            }
        }

        pub fn get_header(&self, name: &str) -> String {
            self.headers_text
                .as_str()
                .split("\n")
                .find(|x| x.starts_with(format!("{}: ", name).as_str()))
                .unwrap()
                .replace(format!("{}: ", name).as_str(), "")
                .to_string()
        }

        pub async fn set_visitor_id(&mut self, client: &Client) {
            let request = client.get(&self.get_header("Origin"));
            let request = self.add_headers(request, false);

            let response = request.send().await.expect("Request failure");
            let response = response
                .text()
                .await
                .expect("Unable to get response body for some reason");
            let captures = Regex::new(r"ytcfg\.set\s*\(\s*(\{.+?\})\s*\)\s*;")
                .unwrap()
                .captures(&response)
                .expect("Unable to parse http response");
            let parsed_json: Value = serde_json::from_str(
                captures
                    .get(1)
                    .expect("Unable to parse visitor id")
                    .as_str(),
            )
            .unwrap();
            self.x_goog_visitor_id =
                Some(parsed_json["VISITOR_DATA"].as_str().unwrap().to_string());
        }

        pub fn set_authorization(&mut self) {
            let since_the_epoch = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Fuck v5")
                .as_secs();
            let mut hasher = Sha1::new();
            hasher.update(
                format!(
                    "{} {} {}",
                    since_the_epoch,
                    self.sapisid,
                    self.get_header("Origin")
                )
                .as_bytes(),
            );
            let hash = hasher.finalize();
            let hash = hash.as_slice().to_vec();
            let hash = hex::encode(&hash);
            self.authorization = Some(format!("SAPISIDHASH {}_{}", since_the_epoch, hash));
        }

        pub fn add_headers(&self, request: RequestBuilder, authorized: bool) -> RequestBuilder {
            let mut headers = HeaderMap::new();
            for line in self.headers_text.as_str().lines() {
                let mut header = line.splitn(2, ": ");
                let (name, value) = (header.next().unwrap(), header.next().unwrap());
                headers.insert(
                    HeaderName::from_str(name).unwrap(),
                    value.to_string().parse().unwrap(),
                );
            }
            headers.remove("Cookie");
            headers.remove("Authorization");
            headers.remove("X-Goog-Visitor-Id");
            if authorized {
                headers.append("Cookie", self.get_header("Cookie").parse().unwrap());
                if self.authorization.is_some() {
                    headers.append(
                        "Authorization",
                        self.authorization.as_ref().unwrap().parse().unwrap(),
                    );
                }
                if self.x_goog_visitor_id.is_some() {
                    headers.append(
                        "X-Goog-Visitor-Id",
                        self.x_goog_visitor_id.as_ref().unwrap().parse().unwrap(),
                    );
                }
            }
            request.headers(headers)
        }
    }

    #[derive(Debug)]
    pub struct Endpoint {
        path: String,
        client: Rc<Client>,
        headers: Rc<RefCell<Headers>>,
    }

    impl Endpoint {
        pub fn new(path: &str, client: Rc<Client>, headers: Rc<RefCell<Headers>>) -> Endpoint {
            Endpoint {
                path: path.to_string(),
                client,
                headers,
            }
        }

        pub fn url(&self) -> String {
            format!("https://music.youtube.com/youtubei/v1/{}?key=AIzaSyC9XL3ZjWddXya6X74dJoCTL-WEYFDNX30&prettyPrint=false", self.path)
        }

        pub async fn make_request(
            &self,
            method: Method,
            body: Body,
        ) -> Result<Response, reqwest::Error> {
            let request = match method {
                Method::GET => self.client.get(&self.url()),
                Method::POST => self.client.post(&self.url()).body(body),
                _ => panic!("Method type {} not supported", method),
            };
            let request = self.headers.borrow().add_headers(request, true);
            request.send().await
        }
    }

    pub fn find_object_by_key(list: &Value, key: &str) -> Option<Value> {
        for item in list.as_array().expect("Object was not an array") {
            let item2 = &item[key];
            if item2.is_object() {
                return Some(item.clone());
            }
        }
        None
    }
}