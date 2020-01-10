use gtk::prelude::*;

pub struct EmptyView {
    pub container: gtk::Grid,
    pub label1: gtk::Label,
    pub label2: gtk::Label,
}

impl EmptyView {
    pub fn new() -> Self {
        let container = gtk::Grid::new();

        let label1 = gtk::Label::new(None);
        label1.set_text("getting things done");

        let label2 = gtk::Label::new(None);
        label2.set_text("getting things done");

        container.attach(&label1, 0, 0, 1, 1);
        container.attach(&label2, 0, 1, 1, 1);
        container.set_row_spacing(12);
        container.set_border_width(6);
        container.set_vexpand(true);
        container.set_hexpand(true);

        EmptyView {
            container,
            label1,
            label2,
        }
    }
}
