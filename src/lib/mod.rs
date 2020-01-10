lazy_static! {
    pub static ref PACKAGE: &'static str = "com.github.ankjevel.tick-tock";
    pub static ref APP_NAME: &'static str = "Tick tock";
    pub static ref APP_NAME_ID: &'static str = {
        Box::leak(
            APP_NAME
                .to_owned()
                .to_lowercase()
                .replace(" ", "-")
                .into_boxed_str(),
        )
    };
}

pub mod config;
pub mod models;
pub mod services;
pub mod views;
