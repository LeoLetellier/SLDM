use iced::widget::{button, column, text, Column};
use crate::app::{UiComponent, Message, App};

#[derive(Default, Debug)]
pub(crate) struct Viewer {
    counter: u32,
}

impl UiComponent for Viewer {
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementViewer => self.counter += 1,
            _ => (),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            text(self.counter),
            button("+").on_press(Message::IncrementViewer),
        ].into()
    }
}