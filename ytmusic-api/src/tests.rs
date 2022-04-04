use crate::structs::*;
use reqwest::Method;
use serde_json::Value;

use super::*;

#[tokio::test]
async fn test_browse_playlists() {
    let mut dir = dirs::home_dir().expect("Unable to locate home dir");
    dir.push("headers.txt");
    let dir = dir.into_os_string().into_string().unwrap();
    let client = YtMusicClient::new(dir.as_str());
    println!("YtMusicClient created, testing headers");
    client.set_headers().await;
    println!("Headers successfully set, testing browse playlists");
    let endpoint = client.endpoint("browse");
    println!("Endpoint created for /youtubei/v1/browse");
    let response = endpoint.make_request(
        Method::POST,
        RequestBody {
            browseId: "FEmusic_liked_playlists".to_string(),
            context: RequestContext::new(),
        }
        .as_body(),
    ).await.expect("Error fetching endpoint");
    println!("Successful response for browse endpoint, attempting to parse");
    let response = response.text().await.expect("Unable to get string response from endpoint");
    let _: Value = serde_json::from_str(&response).expect("Error parsing string response into JSON");
    println!("Successfully parsed response");
}
