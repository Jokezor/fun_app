pub mod ui;

use glib::clone;

use gtk::glib;
use gtk::prelude::*;

fn on_activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    let button = gtk::Button::with_label("Hello world!");

    button.connect_clicked(clone!(@weak window => move |_| window.close()));
    window.set_child(Some(&button));
    window.present();
}

pub fn run() {
    let app = gtk::Application::builder()
        .application_is("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(on_activate);
    app.run();
}
