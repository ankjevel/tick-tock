use gtk::prelude::*;

#[derive(Clone)]
pub struct UserView {
    pub container: gtk::Grid,
    pub label: gtk::Label,
    pub image: gtk::Image,
}

impl UserView {
    pub fn new() -> Self {
        let label = gtk::Label::new(None);
        label.set_markup("");
        label.set_halign(gtk::Align::Center);
        label.set_valign(gtk::Align::Center);
        label.set_vexpand(true);
        label.set_hexpand(true);

        let image = gtk::Image::new();
        image.set_halign(gtk::Align::Center);
        image.set_valign(gtk::Align::Center);
        image.set_vexpand(true);
        image.set_hexpand(true);

        let container = gtk::Grid::new();
        container.set_vexpand(true);
        container.set_hexpand(true);
        container.set_row_spacing(12);
        container.set_border_width(6);

        container.attach(&label, 0, 0, 1, 1);
        container.attach(&image, 0, 1, 1, 1);

        UserView {
            container,
            label,
            image,
        }
    }
}
