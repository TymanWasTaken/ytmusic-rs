use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use sha1::{Sha1, Digest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let headers_file = fs::read_to_string("headers.txt").expect("Fuck");

    let authorization = get_authorization(&headers_file);

    println!("{}", authorization);

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
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.add(&button);

        window.show_all();
    });

    application.run();

    Ok(())
}

fn get_authorization(headers: &String) -> String {
    let cookie_line = headers.split("\n").find(|&x| x.starts_with("Cookie: ")).expect("Fuck v2");
    let sapisid = cookie_line.split(";").find(|&x| x.contains("__Secure-1PSID=")).expect("Fuck v3").trim().split("=").nth(1).expect("Fuck v4");
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Fuck v5")
        .as_secs();
    let mut hasher = Sha1::new();
    hasher.update(format!("{} {} {}", sapisid, "https://music.youtube.com", since_the_epoch).as_bytes());
    let hash = hasher.finalize();
    let hash = hash.as_slice().to_vec();
    let hash = hex::encode(&hash);
    hash
}