// extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
// use gdk_pixbuf::Pixbuf;
use gio::prelude::*;
use gtk::prelude::*;

use dracula::helpers::helper;
use dracula::ui::ui;
use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Dracula");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::None);
    window.set_default_size(350, 70);

    let split_pane = ui::render_ui();
    window.add(&split_pane);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("com.gigitux.dracula"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
