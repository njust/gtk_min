use std::rc::Rc;
use gtk4_helper::{prelude::*, gtk::prelude::*, gtk, glib, gtk::{Application, ApplicationWindow}, gio};
use crate::app::{App, AppMsg, InputData};
use crate::gtk::Orientation;
mod app;


fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.set_accels_for_action("app.search", &["<Ctrl>F"]);
    app.set_accels_for_action("app.scroll", &["<Ctrl>Q"]);
    app.set_accels_for_action("app.prevMatch", &["<Ctrl>P"]);
    app.set_accels_for_action("app.nextMatch", &["<Ctrl>N"]);
    app.set_accels_for_action("app.toggleWrapText", &["<Ctrl>W"]);
    app.set_accels_for_action("app.showPodNames", &["<Alt>P"]);
    app.set_accels_for_action("app.showContainerNames", &["<Alt>C"]);
    app.set_accels_for_action("app.showTimestamps", &["<Alt>T"]);

    app.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title(Some("KTail"));
        window.set_default_size(1200, 700);
        window.set_maximized(true);
        let window = Rc::new(window);

        let global_actions = Rc::new(gio::SimpleActionGroup::new());
        window.insert_action_group("app", Some(&*global_actions));

        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let mut app = App::new_with_data(move |msg| {
            tx.send(msg).expect("Could not send msg");
        }, InputData {
            wnd: window.clone(),
            actions: global_actions.clone()
        });
        window.set_child(Some(app.view()));
        rx.attach(None, move |msg| {
            app.update(msg);
            glib::Continue(true)
        });

        window.show();
    });

    app.run();
}