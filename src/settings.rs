use iced::widget::{button, column, text, Column};
use crate::app::*;

#[derive(Default, Debug)]
pub(crate) struct Settings {
    counter: u32,
}

impl Settings {
    pub(crate) fn update(&mut self, message: Message, handler: &mut ProjectHandler) {
        match message {
            Message::IncrementMenus => handler.counter_one += 1,
            _ => (),
        }
    }

    pub(crate) fn view(&self, handler: &ProjectHandler) -> iced::Element<Message> {
        column![
            text(handler.counter_one),
            button("+").on_press(Message::IncrementMenus),
        ].into()
    }
}