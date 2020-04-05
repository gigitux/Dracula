use gio::prelude::*;
use gtk::prelude::*;

pub fn render_ui() -> gtk::ListBox {
    let listbox = gtk::ListBox::new();
    listbox.set_selection_mode(gtk::SelectionMode::None);
    let model = gtk::ListStore::new(&[String::static_type()]);
    let row = gtk::ListBoxRow::new();

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 50);
    row.add(&hbox);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 50);
    hbox.pack_start(&vbox, true, true, 0);

    let label1 = gtk::Label::new(Some("Automatic Date & Time"));
    let label2 = gtk::Label::new(Some("Requires internet access"));
    vbox.pack_start(&label1, true, true, 0);

    return listbox;
}
