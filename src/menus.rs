use iced::widget::{button, column, text, Column};
use crate::app::*;

#[derive(Default, Debug)]
pub(crate) struct Menus {
    counter: u32,
}

impl UiComponent for Menus {
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementMenus => self.counter += 1,
            _ => (),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            text(self.counter),
            button("+").on_press(Message::IncrementMenus),
        ].into()
    }
}