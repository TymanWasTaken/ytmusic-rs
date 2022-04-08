pub mod structs {
    use reqwest::Body;
    use serde::Serialize;

    #[allow(non_snake_case)]
    #[derive(Serialize, Debug)]
    pub struct RequestClient {
        clientName: String,
        clientVersion: String,
    }

    #[derive(Serialize, Debug)]
    struct RequestUser {}

    #[derive(Serialize, Debug)]
    pub struct RequestContext {
        client: RequestClient,
        user: RequestUser,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Debug)]
    pub struct RequestBody {
        pub browseId: String,
        pub context: RequestContext,
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
                clientVersion: String::from("0.1"),
            }
        }
    }

    impl RequestContext {
        pub fn new() -> RequestContext {
            RequestContext {
                client: RequestClient::new(),
                user: RequestUser::new(),
            }
        }
    }

    impl RequestBody {
        pub fn as_body(&self) -> Body {
            Body::from(serde_json::to_string(self).unwrap())
        }
    }

    #[derive(Debug, Clone)]
    pub struct Playlist {
        pub title: String,
        pub id: String,
        pub thumbnails: Vec<Thumnail>
    }

    #[derive(Debug, Clone)]
    pub struct Thumnail {
        pub height: u16,
        pub width: u16,
        pub url: String
    }
}

pub mod parsing {
    use async_trait::async_trait;
    use reqwest::Response;
    use serde_json::Value;
    use crate::{utils, structs::Playlist, structs::Thumnail};

    pub enum ResponseType {
        LibraryPlaylists
    }

    #[async_trait]
    pub trait YtMusicResponse {
        async fn parse(self, response_type: ResponseType) -> Vec<Playlist>;
    }

    #[async_trait]
    impl YtMusicResponse for Response {
        async fn parse(self, response_type: ResponseType) -> Vec<Playlist> {
            match response_type {
                ResponseType::LibraryPlaylists => {
                    let body = self.text().await.unwrap();
                    let json: Value = serde_json::from_str(&body).unwrap();
                    let val = &json["contents"]["singleColumnBrowseResultsRenderer"]["tabs"][0]["tabRenderer"]["content"]["sectionListRenderer"]["contents"];
                    let val = &utils::find_object_by_key(val, "itemSectionRenderer").unwrap();
                    let val = &val["itemSectionRenderer"]["contents"][0]["gridRenderer"]["items"].as_array().unwrap()[1..];
                    let playlists: Vec<Playlist> = val.iter().map(|item| {
                        let item = &item["musicTwoRowItemRenderer"];
                        Playlist {
                            id: item["title"]["runs"][0]["navigationEndpoint"]["browseEndpoint"]["browseId"].as_str().unwrap().to_string(),
                            title: item["title"]["runs"][0]["text"].as_str().unwrap().to_string(),
                            thumbnails: item["thumbnailRenderer"]["musicThumbnailRenderer"]["thumbnail"]["thumbnails"].as_array().unwrap().iter().map(|thumnail| {
                                Thumnail {
                                    height: thumnail["height"].as_u64().unwrap() as u16,
                                    width: thumnail["width"].as_u64().unwrap() as u16,
                                    url: thumnail["url"].as_str().unwrap().to_string()
                                }
                            }).collect()
                        }
                    }).collect();
                    playlists
                }
            }
        }
    }
}