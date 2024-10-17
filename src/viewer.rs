use iced::widget::{button, column, text, Column};
use crate::app::*;

#[derive(Default, Debug)]
pub(crate) struct Viewer {
    counter: u32,
}

impl Viewer {
    pub(crate) fn update(&mut self, message: Message, handler: &mut ProjectHandler) {
        match message {
            Message::IncrementViewer => handler.focus = OnWindow::Settings,
            _ => (),
        }
    }

    pub(crate) fn view(&self, handler: &ProjectHandler) -> iced::Element<Message> {
        let text_display = match handler.focus {
            OnWindow::Documentation => "Documentation",
            OnWindow::Settings => "Settings",
            OnWindow::WorkingSpace => "Working Space",
        };
        column![
            text(text_display),
            button("+").on_press(Message::IncrementViewer),
        ].into()
    }
}