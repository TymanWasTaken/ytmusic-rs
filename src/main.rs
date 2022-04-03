pub mod ui;

use gtk::prelude::*;
use gtk::Application;
// use reqwest::Method;
// use ytmusic_api::structs::*;
// use ytmusic_api::YtMusicClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // let client = YtMusicClient::new("headers.txt");
    // client.set_headers().await;

    // let result = client
    //     .endpoint("browse")
    //     .make_request(
    //         Method::POST,
    //         RequestBody {
    //             browseId: "FEmusic_liked_playlists".to_string(),
    //             context: RequestContext::new(),
    //         }
    //         .as_body(),
    //     )
    //     .await
    //     .unwrap();

    let application = Application::builder()
        .application_id("tech.tyman.YtMusicRs")
        .build();

    application.connect_activate(ui::setup_ui);

    application.run();

    Ok(())
}
