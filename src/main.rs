use gtk::glib;
use gtk::{ApplicationWindow};
use gtk::prelude::*;
use std::time::Duration;

pub mod time;


// Now change to take input to show on screen

pub fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First gtk program")
            .build();

        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(260, 40);

        let time = time::get_current_date_string();
        let label = gtk::Label::new(None);
        label.set_text(&time);

        window.add(&label);
        window.show_all();

        let tick = move || {
            let time = time::get_current_date_string();
            label.set_text(&time);
            glib::ControlFlow::Continue
        };

        glib::timeout_add_local(Duration::from_millis(500), tick);
    });
    app.run();
}
