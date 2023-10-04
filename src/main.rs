mod config;
mod playerctl;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow,
    gdk_pixbuf::Pixbuf,
    gio::MemoryInputStream,
};

use playerctl::*;

enum GridRows {
    Settings = -1,
    Title = 0,
    Artist = 1,
    Artwork = 2,
    Album = 3,
    Progress = 4,
    Controls = 6,
    Players = 99,
    Refresh = 100
}

fn main() -> glib::ExitCode {
    config::create_config();

    let players: Option<Vec<playerctl::Player>> = playerctl::check_players();

    let app = Application::builder()
        .application_id("com.ewan-selkirk.music-overlay")
        .build();

    app.connect_activate(if players.is_some() {build_ui} else {build_ui_failed});
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Music Overlay"));
    window.set_default_size(320, 600);
    window.set_resizable(false);
    window.set_decorated(false);

    let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .column_homogeneous(true)
        .row_spacing(6)
        .column_spacing(6)
        .width_request(300)
        .height_request(600)
        .build();

    window.set_child(Some(&grid));

    let settings = gtk::Button::with_label("Settings");
    settings.connect_clicked(glib::clone!(@weak app => move |_| {
        build_settings_ui(&app);
    }));
    grid.attach(&settings, 3, GridRows::Settings as i32, 2, 1);

    let title = gtk::Label::new(Some(&call_playerctl("title", None)));
    title.set_css_classes(&vec!["title-2"]);
    title.set_wrap_mode(gtk::pango::WrapMode::Word);
    title.set_wrap(true);
    grid.attach(&title, 0, GridRows::Title as i32, 5, 1);

    let artist = gtk::Label::new(Some(&call_playerctl("artist", None)));
    artist.set_css_classes(&vec!["title-4"]);
    artist.set_wrap_mode(gtk::pango::WrapMode::Word);
    artist.set_wrap(true);
    grid.attach(&artist, 0, GridRows::Artist as i32, 5, 1);

    let album = gtk::Label::new(Some(&call_playerctl("album", None)));
    album.set_wrap_mode(gtk::pango::WrapMode::Word);
    album.set_wrap(true);
    grid.attach(&album, 0, GridRows::Album as i32, 5, 1);

    let artwork = gtk::Image::new();
    // artwork.set_margin_start(4);
    // artwork.set_margin_end(4);
    artwork.set_pixel_size(320);
    artwork.set_from_pixbuf(Some(&get_artwork(&call_playerctl("mpris:artUrl", None))));
    grid.attach(&artwork, 0, GridRows::Artwork as i32, 5, 1);

    /*let progress = gtk::ProgressBar::new();
    progress.set_text(Some("Test Progress Text"));
    progress.set_fraction(432.0 / 1000.0);
    grid.attach(&progress, 0, 3, 5, 1);*/

    let progress = create_progress("1:42", &call_playerctl("mpris:length", Some("duration")));
    progress.set_width_request(320);
    grid.attach(&progress, 0, GridRows::Progress as i32, 5, 2);

    // Media Controls
    let previous_button = gtk::Button::with_label("<<");
    let pause_button = gtk::Button::with_label("||");
    let next_button = gtk::Button::with_label(">>");

    grid.attach(&previous_button, 0, GridRows::Controls as i32, 1, 1);
    grid.attach(&pause_button, 2, GridRows::Controls as i32, 1, 1);
    grid.attach(&next_button, 4, GridRows::Controls as i32, 1, 1);

    let players: [gtk::Button; 2] = [
        gtk::Button::with_label("Spotify"),
        gtk::Button::with_label("VLC")
    ];

    for (i, p) in players.iter().enumerate() {
        if i == 0 {
            p.set_sensitive(false);
        }
        grid.attach(p, 3 * i as i32, GridRows::Players as i32, 5 / 2, 1);
    }

    let refresh = gtk::Button::with_label("Refresh");
    refresh.connect_clicked(|_| println!("Boop"));
    grid.attach(&refresh, 0, GridRows::Refresh as i32, 5, 1);


    window.present();
}

fn get_artwork(url: &str) -> Pixbuf {
    let result = reqwest::blocking::get(url).unwrap();
    let bytes = result.bytes().unwrap().to_vec();
    let bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = MemoryInputStream::from_bytes(&bytes);

    Pixbuf::from_stream(&stream, gtk::gio::Cancellable::NONE).unwrap()
}

fn create_progress(current_progress: &str, end: &str) -> gtk::Grid {
    let progress_grid = gtk::Grid::builder()
        .column_homogeneous(true)
        .row_homogeneous(true)
        .row_spacing(4)
        .width_request(300)
        .build();

    let curr_progress = gtk::Label::new(Some(current_progress));
    curr_progress.set_justify(gtk::Justification::Left);

    let end_progress = gtk::Label::new(Some(end));
    end_progress.set_justify(gtk::Justification::Right);

    let bar = gtk::ProgressBar::new();
    // bar.set_fraction(current_progress.parse::<f64>().unwrap() / end.parse::<f64>().unwrap());
    bar.set_fraction(123450000.0 / 156924000.0);

    progress_grid.attach(&curr_progress, 0, 0, 1, 1);
    progress_grid.attach(&end_progress, 4, 0, 1, 1);
    progress_grid.attach(&bar, 0, 1, 5, 1);

    progress_grid
}

fn build_settings_ui(app: &Application) {
    let window = ApplicationWindow::new(app);

    window.present();
}

fn build_ui_failed(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Music Overlay"));
    window.set_default_size(320, 600);
    window.set_resizable(false);
    window.set_decorated(false);

    let warn = gtk::Label::new(Some("No players could be found...\n:("));
    warn.set_css_classes(&vec!["title-2"]);
    warn.set_justify(gtk::Justification::Center);

    window.set_child(Some(&warn));

    window.present();
}
