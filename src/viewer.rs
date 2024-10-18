use iced::widget::{button, column, text, Column};
use crate::app::*;

#[derive(Default, Debug)]
pub(crate) struct Viewer {
    
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum MessageViewer {
    
}

impl Viewer {
    pub(crate) fn update(&mut self, message: MessageViewer, handler: &mut ProjectHandler) {

    }

    pub(crate) fn view(&self, handler: &ProjectHandler) -> iced::Element<Message> {
        text("viewer").into()
    }
}