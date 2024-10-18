use crate::viewer::{MessageViewer, Viewer};
use crate::settings::{MessageSettings, Settings};
use crate::documentation::{Documentation, MessageDocumentation};
use crate::menus::{MessageMenus, Menus};
use crate::side_panel::{MessageSidePanel, SidePanel};
use src_logic::project::Project;

use iced::{widget::{button, column, row, shader::wgpu::core::device::WaitIdleError, text, Column, center}, Element};
use iced::color;

// Wrap the project object as well as parameters and settings
#[derive(Debug, Default)]
pub(crate) struct ProjectHandler {
    pub(crate) project: Project,
    pub(crate) settings: u16,
}

// All different inputs
#[derive(Debug, Clone, Copy)]
pub(crate) enum Message {
    Menus(MessageMenus),
    SidePanel(MessageSidePanel),
    Viewer(MessageViewer),
}

#[derive(Default, Debug)]
pub(crate) struct App {
    handler: ProjectHandler,
    menus: Menus,
    side_panel: SidePanel,
    viewer: Viewer,
}

impl App {
    pub(crate) fn update(&mut self, message: Message) {
        match message {
            Message::Menus(m) => self.menus.update(m, &mut self.handler),
            Message::SidePanel(m) => self.side_panel.update(m, &mut self.handler),
            Message::Viewer(m) => self.viewer.update(m, &mut self.handler),
        }
    }

    pub(crate) fn view(&self) -> Element<Message> {
        let menus_view = self.menus.view(&self.handler).explain(color!(0x0000ff));
        let side_panel_view = self.side_panel.view(&self.handler).explain(color!(0x0000ff));
        let viewer_view = self.viewer.view(&self.handler).explain(color!(0x0000ff));

        let layout = column![menus_view, row![side_panel_view, viewer_view]];
        
        layout.into()
    }
}

impl App {
    pub(crate) fn title(&self) -> String {
        format!("Slide Displacement Analysis - {}", self.handler.project.title)
    }
}
