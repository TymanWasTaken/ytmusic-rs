pub mod structs {
    use reqwest::Body;
    use serde::Serialize;

    #[allow(non_snake_case)]
    #[derive(Serialize, Debug)]
    pub struct RequestClient {
        clientName: String,
        clientVersion: String
    }

    #[derive(Serialize, Debug)]
    struct RequestUser {}

    #[derive(Serialize, Debug)]
    pub struct RequestContext {
        client: RequestClient,
        user: RequestUser
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Debug)]
    pub struct RequestBody {
        pub browseId: String,
        pub context: RequestContext
    }

    impl RequestUser {
        pub fn new() -> RequestUser {
            RequestUser {}
        }
    }

    impl RequestClient {
        pub fn new() -> RequestClient {
            RequestClient {
                clientName: String::from("WEB_REMIX"),
                clientVersion: String::from("0.1")
            }
        }
    }

    impl RequestContext {
        pub fn new() -> RequestContext {
            RequestContext {
                client: RequestClient::new(),
                user: RequestUser::new()
            }
        }
    }

    impl RequestBody {
        pub fn as_body(&self) -> Body {
            Body::from(serde_json::to_string(self).unwrap())
        }
    }
}