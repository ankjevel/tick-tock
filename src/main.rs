#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate hmac_sha256;
extern crate md5;
extern crate reqwest;
extern crate serde;

mod lib;

use gio::prelude::*;
use gtk::prelude::*;
use lib::{
    config::CONFIG,
    services::{file, tenkfeet},
    views::widgets::Widgets,
    {APP_NAME, PACKAGE},
};
use std::{cell::RefCell, env::args, rc::Rc, thread};

pub fn main() {
    glib::set_program_name(Some(&APP_NAME));

    let application = gtk::Application::new(Some(&PACKAGE), gio::ApplicationFlags::empty())
        .expect("initialization failed");

    application.connect_startup(|app| {
        let application = Application::new(app);

        let application_container = RefCell::new(Some(application));
        app.connect_shutdown(move |_| {
            let application = application_container
                .borrow_mut()
                .take()
                .expect("Shutdown called multiple times");
            drop(application);
        });
    });

    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());
}

pub struct Application {
    pub widgets: Rc<Widgets>,
}

enum Message {
    UpdateUser(String, String),
}

impl Application {
    pub fn new(app: &gtk::Application) -> Self {
        let app = Application {
            widgets: Rc::new(Widgets::new(app)),
        };

        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        app.update_main_ui_thread(rx);

        app.get_user(tx.clone());

        app
    }

    fn update_main_ui_thread(&self, rx: glib::Receiver<Message>) {
        let widgets = self.widgets.clone();
        rx.attach(None, move |msg| {
            match msg {
                Message::UpdateUser(first_name, thumbnail) => {
                    widgets.user_view.label.set_markup(&first_name);

                    let url = file::get_file(&thumbnail).unwrap();

                    widgets.user_view.image.set_from_file(&url);

                    widgets
                        .view_stack
                        .set_visible_child(&widgets.user_view.container);
                }
            }
            glib::Continue(true)
        });
    }

    fn get_user(&self, tx: glib::Sender<Message>) {
        thread::spawn(move || {
            match tenkfeet::get_user() {
                Ok(user) => {
                    thread::sleep(std::time::Duration::from_secs(2));
                    if CONFIG.debug {
                        println!("{:?}", user);
                    }

                    let _ = tx.send(Message::UpdateUser(
                        user.first_name.to_string(),
                        user.thumbnail.to_string(),
                    ));
                }
                Err(err) => panic!("{:?}", err),
            };
        });
    }
}
