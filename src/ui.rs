use glib::clone;

use gtk::glib;
use gtk::{ApplicationWindow, Button};
use gtk::prelude::*;


// Now change to take input to show on screen

pub fn main(current_time: String) {
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First gtk program")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label(&current_time);
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.add(&button);

        window.show_all();
    });
    app.run();
}
