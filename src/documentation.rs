use iced::widget::{button, column, text, Column};
use crate::app::*;

#[derive(Default, Debug)]
pub(crate) struct Documentation {
    counter: u32,
}

impl Documentation {
    pub(crate) fn update(&mut self, message: Message, handler: &mut ProjectHandler) {
        match message {
            Message::IncrementMenus => self.counter += 1,
            _ => (),
        }
    }

    pub(crate) fn view(&self, handler: &ProjectHandler) -> iced::Element<Message> {
        column![
            text(self.counter),
            button("+").on_press(Message::IncrementMenus),
        ].into()
    }
}