use iced::widget::{button, column, container, row, text, Button};
use iced::{Length, Alignment};
use crate::app::*;

#[derive(Debug, Clone, Copy)]
pub(crate) enum MessageMenus {
}

#[derive(Default, Debug)]
pub(crate) struct Menus {
}

impl Menus {
    pub(crate) fn update(&mut self, message: MessageMenus, handler: &mut ProjectHandler) {
        match message {
            _ => (),
        }
    }

    pub(crate) fn view(&self, handler: &ProjectHandler) -> iced::Element<Message> {
        text("Menus").into()
    }
}
