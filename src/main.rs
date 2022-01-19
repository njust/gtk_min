use std::rc::Rc;
use gtk::{Application, ApplicationWindow, Orientation, prelude::*};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let window = Rc::new(ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!!")
            .build());

        let toolbar = gtk::Box::new(Orientation::Horizontal, 0);
        let btn = gtk::Button::with_label("Open");
        toolbar.append(&btn);

        let buffer = sourceview5::Buffer::new(None);
        let view = sourceview5::View::builder()
            .buffer(&buffer)
            .monospace(true)
            .show_line_numbers(true)
            .highlight_current_line(true)
            .hexpand(true)
            .vexpand(true)
            .build();

        let container = gtk::Box::new(Orientation::Vertical, 0);
        let spinner = gtk::Spinner::new();
        spinner.set_height_request(32);
        spinner.set_width_request(32);
        spinner.start();

        container.append(&spinner);
        container.append(&toolbar);
        container.append(&view);

        window.set_child(Some(&container));
        window.show();
    });

    app.run();
}