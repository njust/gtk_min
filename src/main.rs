use gtk4_helper::{
    gtk::prelude::*,
    gtk::{Application, ApplicationWindow}
};
use sourceview5::prelude::*;


fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!!")
            .build();

        let buffer = sourceview5::Buffer::new(None);
        let view = sourceview5::View::with_buffer(&buffer);
        view.set_monospace(true);
        view.set_background_pattern(sourceview5::BackgroundPatternType::Grid);
        view.set_show_line_numbers(true);
        view.set_highlight_current_line(true);
        view.set_tab_width(4);
        view.set_hexpand(true);

        window.set_child(Some(&view));
        window.show();
    });

    app.run();
}