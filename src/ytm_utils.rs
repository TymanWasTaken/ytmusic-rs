pub mod utils {
    use hyper::client::HttpConnector;
    use hyper::header::HeaderValue;
    use hyper::http::request::Builder;
    use hyper::{Body, Client, Method, Request, Response};
    use hyper_tls::HttpsConnector;
    use regex::Regex;
    use sha1::{Digest, Sha1};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug)]
    pub struct Headers {
        authorization: Option<String>,
        user_agent: String,
        accept: String,
        accept_encoding: String,
        content_type: String,
        content_encoding: String,
        origin: String,
        x_goog_visitor_id: Option<String>,
        sapisid: String,
    }

    impl Headers {
        pub fn new(data: &String) -> Headers {
            let cookie_line = data
                .split("\n")
                .find(|&x| x.starts_with("Cookie: "))
                .expect("Fuck v2");
            let sapisid = cookie_line
                .split(";")
                .find(|&x| x.contains("__Secure-1PSID="))
                .expect("Fuck v3")
                .trim()
                .split("=")
                .nth(1)
                .expect("Fuck v4");

            Headers {
                authorization: None,
                user_agent:
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:88.0) Gecko/20100101 Firefox/88.0"
                        .to_string(),
                accept: "*/*".to_string(),
                accept_encoding: "gzip, deflate".to_string(),
                content_type: "application/json".to_string(),
                content_encoding: "gzip".to_string(),
                origin: "https://music.youtube.com".to_string(),
                x_goog_visitor_id: None,
                sapisid: sapisid.to_string(),
            }
        }

        pub async fn set_visitor_id(&mut self, client: &Client<HttpsConnector<HttpConnector>>) {
            let request = Request::builder().method(Method::GET).uri(&self.origin);
            let request = self
                .add_headers(request, false)
                .body(Body::empty())
                .expect("Error building request");

            let response = client.request(request).await.expect("Request failure");
            dbg!(response.headers());
            // let response = hyper::body::to_bytes(response.into_body()).await.unwrap();
            // let response = String::from_utf8(response.into_iter().collect()).expect("Invalid http response");
            // let captures = Regex::new(r"ytcfg\.set\s*\(\s*(\{.+?\})\s*\)\s*;").unwrap().captures(&response).expect("Unable to parse http response");
            // self.x_goog_visitor_id = Some(captures.get(1).expect("Unable to parse visitor id").as_str().to_string());
        }

        pub fn set_authorization(&mut self) {
            let since_the_epoch = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Fuck v5")
                .as_secs();
            let mut hasher = Sha1::new();
            hasher
                .update(format!("{} {} {}", since_the_epoch, self.sapisid, self.origin).as_bytes());
            let hash = hasher.finalize();
            let hash = hash.as_slice().to_vec();
            let hash = hex::encode(&hash);
            self.authorization = Some(format!("SAPISIDHASH {}_{}", since_the_epoch, hash));
        }

        pub fn add_headers(&self, mut request: Builder, authorized: bool) -> Builder {
            let headers = request.headers_mut().unwrap();
            if self.authorization.is_some() && authorized {
                headers.insert(
                    "Authorization",
                    HeaderValue::from_str(&self.authorization.as_ref().unwrap()).unwrap(),
                );
            }
            if self.x_goog_visitor_id.is_some() && authorized {
                headers.insert(
                    "X-Goog-Visitor-Id",
                    HeaderValue::from_str(&self.x_goog_visitor_id.as_ref().unwrap()).unwrap(),
                );
            }
            headers.insert(
                "User-Agent",
                HeaderValue::from_str(&self.user_agent).unwrap(),
            );
            headers.insert("Accept", HeaderValue::from_str(&self.accept).unwrap());
            headers.insert(
                "Accept-encoding",
                HeaderValue::from_str(&self.accept_encoding).unwrap(),
            );
            headers.insert(
                "Content-Type",
                HeaderValue::from_str(&self.content_type).unwrap(),
            );
            headers.insert(
                "Content-Encoding",
                HeaderValue::from_str(&self.content_encoding).unwrap(),
            );
            headers.insert("Origin", HeaderValue::from_str(&self.origin).unwrap());
            request
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
            client: &Client<HttpConnector>,
            method: Method,
            headers: &Headers,
            body: Body,
        ) -> Response<Body> {
            let request = Request::builder().method(method).uri(self.url());
            let request = headers
                .add_headers(request, true)
                .body(body)
                .expect("Error building request");

            client.request(request).await.expect("Request failure")
        }
    }
}
