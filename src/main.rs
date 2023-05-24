mod components;
#[rustfmt::skip]
mod config;
mod localize;
mod process;
mod subscriptions;
use config::APP_ID;
use log::info;

use localize::localize;

use crate::{components::app, config::VERSION};

fn main() -> cosmic::iced::Result {
    // Initialize logger
    pretty_env_logger::init();
    info!("cosmic-launcher ({})", APP_ID);
    info!("Version: {} ({})", VERSION, config::profile());

    // Prepare i18n
    localize();

    app::run()
}
