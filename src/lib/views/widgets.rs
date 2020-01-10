use crate::lib::{
    views::{empty_view::EmptyView, header::Header, user_view::UserView},
    {APP_NAME, APP_NAME_ID},
};

use gtk::prelude::*;

pub struct Widgets {
    pub window: gtk::ApplicationWindow,
    pub header: Header,
    pub view_stack: gtk::Stack,
    pub empty_view: EmptyView,
    pub user_view: UserView,
}

impl Widgets {
    pub fn new(application: &gtk::Application) -> Self {
        let user_view = UserView::new();
        let empty_view = EmptyView::new();

        let view_stack = gtk::Stack::new();
        view_stack.set_border_width(6);
        view_stack.set_vexpand(true);
        view_stack.set_hexpand(true);
        view_stack.add(&empty_view.container);
        view_stack.add(&user_view.container);

        let header = Header::new();

        let window = gtk::ApplicationWindow::new(application);
        window.set_icon_name(Some(&APP_NAME));
        window.set_property_window_position(gtk::WindowPosition::Center);
        window.set_titlebar(Some(&header.container));
        window.add(&view_stack);
        window.set_wmclass(&APP_NAME_ID, &APP_NAME);
        window.show_all();

        Widgets {
            window,
            header,
            view_stack,
            empty_view,
            user_view,
        }
    }
}
