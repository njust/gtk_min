use gtk4_helper::{
    gtk,
    gtk::prelude::*,
    gtk::{Application, ApplicationWindow}
};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        window.show();
    });

    app.run();
}