use iced::widget::{button, column, row, text};
use iced::{Length, Alignment};
use crate::app::*;

#[derive(Default, Debug)]
pub(crate) struct Settings {
    
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum MessageSettings {

}

impl Settings {
    pub(crate) fn update(&mut self, message: MessageSettings, handler: &mut ProjectHandler) {
        match message {
            _ => (),
        }
    }

    pub(crate) fn view(&self, handler: &ProjectHandler) -> iced::Element<Message> {
        text("Settings").into()
    }
}