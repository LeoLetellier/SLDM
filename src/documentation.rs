use iced::widget::{button, column, text, row};
use iced::{Length, Alignment};
use crate::app::*;

#[derive(Default, Debug)]
pub(crate) struct Documentation {
    
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum MessageDocumentation {
    
}

impl Documentation {
    pub(crate) fn update(&mut self, message: MessageDocumentation, handler: &mut ProjectHandler) {
        match message {
            _ => (),
        }
    }

    pub(crate) fn view(&self, handler: &ProjectHandler) -> iced::Element<Message> {
        text("Documentation").into()
    }
}