extern crate gtk;
extern crate gdk;
extern crate cairo;

mod core;
mod view;

use gtk::traits::*;
use gtk::signal::Inhibit;

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    let button = gtk::Button::new_with_label("Click me!").unwrap();
    let term = self::view::TermView::new();
    window.add(term.as_widget());
    button.connect_clicked(|_| {
        gtk::main_quit();
    });

    window.show_all();
    gtk::main();
}
