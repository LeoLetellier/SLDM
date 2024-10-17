use iced::{widget::{button, column, row, shader::wgpu::core::device::WaitIdleError, text, Column}, Element};
use crate::menus::Menus;
use crate::explorer::Explorer;
use crate::viewer::Viewer;
use crate::settings::Settings;
use crate::documentation::Documentation;
use src_logic::project::Project;

// Wrap the project object as well as parameters and settings
#[derive(Debug, Default)]
pub(crate) struct ProjectHandler {
    pub(crate) project: Project,
    pub(crate) focus: OnWindow,
    pub(crate) counter_one: u16,
    pub(crate) counter_two: u16,
}

// Capture which screen has to be displayed
#[derive(Default, Debug)]
pub(crate) enum OnWindow {
    #[default]
    WorkingSpace,
    Settings,
    Documentation,
}

// All different inputs
#[derive(Debug, Clone, Copy)]
pub(crate) enum Message {
    IncrementMenus,
    IncrementExplorer,
    IncrementViewer,
}

#[derive(Default, Debug)]
pub(crate) struct App {
    handler: ProjectHandler,
    working_space: WorkingSpace,
    settings: Settings,
    documentation: Documentation,
}

impl App {
    pub(crate) fn update(&mut self, message: Message) {
        self.working_space.update(message, &mut self.handler);
        self.settings.update(message, &mut self.handler);
        
    }

    pub(crate) fn view(&self) -> Element<Message> {
        let display_focus = match self.handler.focus {
            OnWindow::WorkingSpace => self.working_space.view(&self.handler),
            OnWindow::Documentation => self.documentation.view(&self.handler),
            OnWindow::Settings => self.settings.view(&self.handler),
        };
        display_focus.into()
    }
}

impl App {
    pub(crate) fn title(&self) -> String {
        format!("Slide Displacement Analysis - {}", self.handler.project.title)
    }
}

#[derive(Default, Debug)]
pub(crate) struct WorkingSpace {
    menus: Menus,
    explorer: Explorer,
    viewer: Viewer,
}

impl WorkingSpace {
    pub(crate) fn update(&mut self, message: Message, handler: &mut ProjectHandler) {
        self.menus.update(message, handler);
        self.explorer.update(message, handler);
        self.viewer.update(message, handler);
    }

    pub(crate) fn view(&self, handler: &ProjectHandler) -> Element<Message> {
        column![
            self.menus.view(handler),
            row![self.explorer.view(handler), self.viewer.view(handler)]
        ].into()
    }
}
