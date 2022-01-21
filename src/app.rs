use std::rc::Rc;
use sourceview5::prelude::*;
use gtk4_helper::{
    component::{Command, Component, MsgHandler},
    gtk::{self, prelude::*},
};
use gtk4_helper::gtk::{ApplicationWindow, Orientation};


#[derive(Clone)]
pub enum AppMsg {
    Open,
    Selected(Option<String>),
}

pub struct App {
    container: gtk::Box,
    wnd: Rc<ApplicationWindow>,
}

impl Component for App {
    type Msg = AppMsg;
    type View = gtk::Box;
    type Input = Rc<ApplicationWindow>;

    fn create<T: MsgHandler<Self::Msg> + Clone>(sender: T, input: Option<Self::Input>) -> Self {
        let toolbar = gtk::Box::new(Orientation::Horizontal, 0);
        let btn = gtk::Button::with_label("Open");
        let tx = sender.clone();
        btn.connect_clicked(move|_| {
            tx(AppMsg::Open);
        });
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
        let wnd = input.expect("No wnd!");
        container.append(&toolbar);
        container.append(&view);
        Self {
            container,
            wnd
        }
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            AppMsg::Open => {
                return self.run_async_local(load(self.wnd.clone()));
            }
            AppMsg::Selected(path) => {
                println!("Selected: {:?}", path);
            }
        }
        Command::None
    }

    fn view(&self) -> &Self::View {
        &self.container
    }
}

async fn load(wnd: Rc<ApplicationWindow>) -> AppMsg {
    let dlg = gtk::FileChooserDialog::builder()
        .title("Select file")
        .modal(true)
        .transient_for(&*wnd)
        .action(gtk::FileChooserAction::Open)
        .build();

    dlg.add_buttons(&[("Select", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
    let path = if dlg.run_future().await == gtk::ResponseType::Ok {
        dlg.file().and_then(|sel| sel.path())
    } else {
        None
    };

    dlg.close();
    AppMsg::Selected(path.as_ref().and_then(|p| p.to_str()).map(|s| s.to_string()))
}