use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

pub fn setup_ui(app: &Application) {
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
