use crate::lib::APP_NAME;
use gtk::prelude::*;

pub struct Header {
    pub container: gtk::HeaderBar,
}

impl Header {
    pub fn new() -> Self {
        let container = gtk::HeaderBar::new();
        container.set_title(Some(&APP_NAME));
        container.set_show_close_button(true);

        Header { container }
    }
}
