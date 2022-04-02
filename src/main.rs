pub mod ytm_utils;

// use gtk::prelude::*;
// use gtk::{Application, ApplicationWindow, Button};
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::fs;
use ytm_utils::utils::Headers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let headers = fs::read_to_string("headers.txt").expect("Fuck");
    let mut headers = Headers::new(&headers);

    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

    dbg!(headers.set_visitor_id(&client).await);
    dbg!(headers.set_authorization());
    dbg!(headers);

    // let application = Application::builder()
    //     .application_id("tech.tyman.YtMusicRs")
    //     .build();

    // application.connect_activate(|app| {
    //     let window = ApplicationWindow::builder()
    //         .application(app)
    //         .title("YouTube Music")
    //         .default_width(350)
    //         .default_height(70)
    //         .build();

    //     let button = Button::with_label("Click me!");
    //     button.connect_clicked(|_| {
    //         eprintln!("Clicked!");
    //     });
    //     window.add(&button);

    //     window.show_all();
    // });

    // application.run();

    Ok(())
}
