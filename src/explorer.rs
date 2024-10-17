use iced::widget::{button, column, text, Column};
use crate::app::*;

#[derive(Default, Debug)]
pub(crate) struct Explorer {
    counter: u32,
}

impl Explorer {
    pub(crate) fn update(&mut self, message: Message, handler: &mut ProjectHandler) {
        match message {
            Message::IncrementExplorer => handler.counter_two += 1,
            _ => (),
        }
    }

    pub(crate) fn view(&self, handler: &ProjectHandler) -> iced::Element<Message> {
        column![
            text(handler.counter_two),
            button("+").on_press(Message::IncrementExplorer),
        ].into()
    }
}