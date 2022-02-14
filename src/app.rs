use std::rc::Rc;
use sourceview5::prelude::*;
use gtk4_helper::{component::{Command, Component, MsgHandler}, gio, glib, gtk::{self, prelude::*}};
use gtk4_helper::gio::SimpleActionGroup;
use gtk4_helper::gtk::{ApplicationWindow, Orientation};
use gtk4_helper::model::prelude::*;
pub const DEFAULT_MARGIN: i32 = 4;

#[derive(Clone)]
pub enum AppMsg {
    Open,
    Selected(Option<String>),
}

pub struct App {
    container: gtk::Box,
    wnd: Rc<ApplicationWindow>,
}

pub struct InputData {
    pub wnd: Rc<ApplicationWindow>,
    pub actions: Rc<SimpleActionGroup>,
}

impl Component for App {
    type Msg = AppMsg;
    type View = gtk::Box;
    type Input = InputData;

    fn create<T: MsgHandler<Self::Msg> + Clone>(sender: T, input: Option<Self::Input>) -> Self {
        let toolbar = gtk::Box::new(Orientation::Horizontal, 0);
        let btn = gtk::Button::with_label("Open");
        let tx = sender.clone();
        btn.connect_clicked(move|_| {
            tx(AppMsg::Open);
        });
        toolbar.append(&btn);

        let input = input.expect("No wnd!");

        let settings: Settings = Settings::default();
        let settings = settings.to_object();
        add_log_view_settings_menu(input.actions, &toolbar, &settings, sender.clone());

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
        let wnd = input.wnd;
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

#[model]
struct Settings {
    #[field]
    wrap_text: bool,
    #[field]
    show_pod_names: bool,
    #[field]
    show_container_names: bool,
    #[field]
    show_timestamps: bool,
}

fn add_log_view_settings_menu<T: MsgHandler<AppMsg> + Clone>(action_group: Rc<SimpleActionGroup>, toolbar: &gtk::Box, settings_obj: &glib::Object, sender: T) {
    let menu = gio::Menu::new();
    menu.append(Some("Wrap lines"), Some("app.toggleWrapText"));
    menu.append(Some("Show pod names"), Some("app.showPodNames"));
    menu.append(Some("Show container names "), Some("app.showContainerNames"));
    menu.append(Some("Show timestamps"), Some("app.showTimestamps"));

    let menu_btn =gtk::MenuButtonBuilder::new()
        .icon_name("emblem-system-symbolic")
        .menu_model(&menu)
        .margin_end(DEFAULT_MARGIN)
        .build();

    add_property_action(&action_group, "toggleWrapText", settings_obj, Settings::wrap_text, || AppMsg::Open, sender.clone());
    add_property_action(&action_group, "showContainerNames", settings_obj, Settings::show_container_names, || AppMsg::Open, sender.clone());
    add_property_action(&action_group, "showTimestamps", settings_obj, Settings::show_timestamps, || AppMsg::Open, sender.clone());
    add_property_action(&action_group, "showPodNames", settings_obj, Settings::show_pod_names, || AppMsg::Open, sender.clone());
    toolbar.append(&menu_btn);
}

fn add_property_action<T: MsgHandler<AppMsg> + Clone, M: 'static + Fn() -> AppMsg>(
    action_group: &gio::SimpleActionGroup,
    name: &str,
    settings_obj: &glib::Object,
    property_name: &str,
    msg: M,
    tx: T
) {
    let action = gio::PropertyAction::new(name, settings_obj, property_name);
    action.connect_state_notify(move |_| {
        tx(msg());
    });
    action_group.add_action(&action);
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