pub mod utils {
    use reqwest::header::HeaderMap;
    use reqwest::{Client, RequestBuilder, Method, Body, Response};
    use sha1::{Digest, Sha1};
    use std::time::{SystemTime, UNIX_EPOCH};
    use regex::Regex;
    use serde_json::Value;

    #[derive(Debug)]
    pub struct Headers {
        x_goog_visitor_id: Option<String>,
        authorization: Option<String>,
        sapisid: String,
        headers_text: String
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
                headers_text: data.to_string()
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
            let response = response.text().await.expect("Unable to get response body for some reason");
            let captures = Regex::new(r"ytcfg\.set\s*\(\s*(\{.+?\})\s*\)\s*;").unwrap().captures(&response).expect("Unable to parse http response");
            let parsed_json: Value = serde_json::from_str(captures.get(1).expect("Unable to parse visitor id").as_str()).unwrap();
            self.x_goog_visitor_id = Some(parsed_json["VISITOR_DATA"].as_str().unwrap().to_string());
        }

        pub fn set_authorization(&mut self) {
            let since_the_epoch = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Fuck v5")
                .as_secs();
            let mut hasher = Sha1::new();
            hasher
                .update(format!("{} {} {}", since_the_epoch, self.sapisid, self.get_header("Origin")).as_bytes());
            let hash = hasher.finalize();
            let hash = hash.as_slice().to_vec();
            let hash = hex::encode(&hash);
            self.authorization = Some(format!("SAPISIDHASH {}_{}", since_the_epoch, hash));
        }

        pub fn add_headers(&self, request: RequestBuilder, authorized: bool) -> RequestBuilder {
            let mut headers = HeaderMap::new();
            self.headers_text.lines().for_each(|line| {
                let header = line.splitn(1,": ");
                let (name, value) = (header.nth(0).unwrap(), header.nth(1).unwrap());
                headers.insert(name, value.parse().unwrap());
            });
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
            } else {
                headers.remove("Cookie");
                headers.remove("Authorization");
                headers.remove("X-Goog-Visitor-Id");
            }
            request.headers(headers)
        }
    }

    #[derive(Debug)]
    pub struct Endpoint {
        pub path: String,
    }

    impl Endpoint {
        pub fn url(&self) -> String {
            format!("https://music.youtube.com/youtubei/v1/{}?alt=json&key=AIzaSyC9XL3ZjWddXya6X74dJoCTL-WEYFDNX30", self.path)
        }

        pub async fn make_request(
            &self,
            client: &Client,
            method: Method,
            headers: &Headers,
            body: Body,
        ) -> Result<Response, reqwest::Error> {
            let request = match method {
                Method::GET => client.get(&self.url()),
                Method::POST => client.post(&self.url()).body(body),
                _ => panic!("Method type {} not supported", method)
            };
            let request = headers.add_headers(request, true);
            dbg!(&request);
            request.send().await
        }
    }
}
