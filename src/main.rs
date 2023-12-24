mod app;
mod logger;
mod text_editor;
mod theme;

use log::info;
use logger::setup_logger;

use crate::app::App;

fn main() {
    setup_logger();

    info!("This file is the log file");
    info!("I choosed to be opened by default because why not");
    info!("hjkl or arrow keys to move and enjoy playing around for 20 seconds and then be bored");

    App::new().run();
}
