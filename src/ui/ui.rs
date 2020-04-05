use gio::prelude::*;
use gtk::prelude::*;
use gtk::{CellRendererPixbuf, CellRendererText, Orientation, TreeStore, TreeView, TreeViewColumn};

pub fn render_ui() -> gtk::Box {
    // right pane
    let right_tree = TreeView::new();
    let right_column_types = [String::static_type(), String::static_type()];
    let right_store = TreeStore::new(&right_column_types);
    let renderer = CellRendererPixbuf::new();
    let icon = TreeViewColumn::new();
    let name = TreeViewColumn::new();

    icon.set_title("Picture");
    name.set_title("name");

    icon.pack_start(&renderer, false);

    icon.add_attribute(&renderer, "pixbuf", 0);

    let renderer2 = CellRendererText::new();
    icon.pack_start(&renderer2, true);

    right_tree.append_column(&icon);
    right_tree.append_column(&name);
    right_tree.set_model(Some(&right_store));
    right_tree.set_headers_visible(true);

    // display the panes

    let split_pane = gtk::Box::new(Orientation::Horizontal, 100);

    split_pane.set_size_request(0, 0);
    split_pane.add(&right_tree);

    return split_pane;
}
