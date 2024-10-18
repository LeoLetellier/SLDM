#![windows_subsystem = "windows"]
mod app;
mod menus;
mod side_panel;
mod viewer;
mod settings;
mod documentation;

use app::App;
use iced::{Size, Theme};

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
    .theme(|_| Theme::Nord)
    .centered()
    .window_size(Size::new(1280., 720.))
    .run()
}