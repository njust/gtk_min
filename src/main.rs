use std::rc::Rc;
use gtk4_helper::{
    prelude::*,
    gtk::prelude::*,
    gtk,
    glib,
    gtk::{Application, ApplicationWindow}
};
use crate::app::{App, AppMsg};
use crate::gtk::Orientation;
mod app;


fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();
    app.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title(Some("KTail"));
        window.set_default_size(1200, 700);
        window.set_maximized(true);
        let window = Rc::new(window);


        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let mut app = App::new_with_data(move |msg| {
            tx.send(msg).expect("Could not send msg");
        }, window.clone());
        window.set_child(Some(app.view()));
        rx.attach(None, move |msg| {
            app.update(msg);
            glib::Continue(true)
        });

        window.show();
    });

    app.run();
}