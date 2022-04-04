use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
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

    application.connect_activate(build_ui);

    application.run();

    Ok(())
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("YouTube Music")
        // .default_width(350)
        // .default_height(70)
        .build();

    // let button = Button::with_label("Click me!");
    // window.set_child(Some(&button));

    window.present();
}
