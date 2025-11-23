use gtk::glib;
use gtk::{ApplicationWindow};
use gtk::{gdk};
use gtk::prelude::*;
use std::time::Duration;

pub mod time;


// Now change to take input to show on screen

pub fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();

    app.connect_startup(|app| {
        let provider = gtk::CssProvider::new();

        // load css
        let style = include_bytes!("style.css");
        provider.load_from_bytes(style).expect("Failed to load CSS");

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );
    });

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First gtk program")
            .build();

        let time = time::get_current_date_string();
        let label = gtk::Label::new(None);
        label.set_text(&time);

        window.set_widget_name("clock-face");
        label.set_widget_name("clock-text");

        window.add(&label);
        window.show();

        let tick = move || {
            let time = time::get_current_date_string();
            label.set_text(&time);
            glib::ControlFlow::Continue
        };

        glib::timeout_add_local(Duration::from_millis(500), tick);
    });
    app.run();
}
