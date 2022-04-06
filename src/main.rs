mod ui;
mod threading;

use gtk::prelude::*;
use gtk::Application;

fn main() {
    let application = Application::builder()
        .application_id("tech.tyman.YtMusicRs")
        .build();

    application.connect_activate(ui::setup_ui);

    application.run();
}
