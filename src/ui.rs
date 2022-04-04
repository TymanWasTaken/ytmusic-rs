use gtk::{prelude::*, Application, ApplicationWindow, Box, Grid, Label};

pub fn setup_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("YouTube Music")
        .default_width(350)
        .default_height(70)
        .build();

    let r#box = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .build();

    let label = Label::builder().label("Playlists").margin_top(10).build();
    r#box.append(&label);

    let grid = Grid::builder()
        .column_homogeneous(true)
        .row_homogeneous(true)
        .build();
    grid.attach(&Label::builder().label("Playlist 1").build(), 0, 0, 1, 1);
    grid.attach(&Label::builder().label("Pog playlist").build(), 1, 0, 1, 1);

    grid.attach(&Label::builder().label("Playlist 2").build(), 0, 1, 1, 1);
    grid.attach(&Label::builder().label("ae").build(), 1, 1, 1, 1);

    grid.attach(&Label::builder().label("Playlist 3").build(), 0, 2, 1, 1);
    grid.attach(
        &Label::builder().label("I hope this works").build(),
        1,
        2,
        1,
        1,
    );
    r#box.append(&grid);

    window.set_child(Some(&r#box));
    window.present();
}
