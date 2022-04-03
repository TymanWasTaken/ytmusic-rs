pub mod ytm_utils;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use reqwest::{Client, Method, Body};
use std::fs;
use ytm_utils::utils::{Headers, Endpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let headers = fs::read_to_string("headers.txt").expect("Fuck");
    let mut headers = Headers::new(&headers);

    let client = Client::new();

    headers.set_visitor_id(&client).await;
    headers.set_authorization();

    let result = Endpoint {
        path: "browse".to_string()
    }.make_request(&client, Method::POST, &headers, Body::from("{\"browseId\": \"FEmusic_liked_playlists\"}")).await.unwrap();
    dbg!(result.text().await.unwrap());

    let application = Application::builder()
        .application_id("tech.tyman.YtMusicRs")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("YouTube Music")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Click me!");
        window.add(&button);

        window.show_all();
    });

    application.run();

    Ok(())
}
