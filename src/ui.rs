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
    
    for (i, playlist) in playlists.iter().enumerate() {
        grid.attach(&Label::builder().label(&format!("Playlist {}", i)).build(), 0, i.try_into().unwrap(), 1, 1);
        grid.attach(&Label::builder().label(&playlist.title).build(), 1, i.try_into().unwrap(), 1, 1);
    }
        
    r#box.append(&grid);

    window.set_child(Some(&r#box));
    window.present();
}
