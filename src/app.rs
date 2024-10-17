use iced::{widget::{button, column, row, shader::wgpu::core::device::WaitIdleError, text, Column}, Element};
use crate::menus::Menus;
use crate::explorer::Explorer;
use crate::viewer::Viewer;
use crate::settings::Settings;
use crate::documentation::Documentation;

#[derive(Default, Debug)]
pub(crate) enum OnWindow {
    #[default]
    WorkingSpace,
    Settings,
    Documentation,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Message {
    IncrementMenus,
    IncrementExplorer,
    IncrementViewer,
}

pub(crate) trait UiComponent {
    fn update(&mut self, message: Message);

    fn view(&self) -> Element<Message>;
}

#[derive(Default, Debug)]
struct Project {
    title: String,
}

#[derive(Default, Debug)]
pub(crate) struct App {
    pub(crate) focus: OnWindow,
    project: Project,
    working_space: WorkingSpace,
    settings: Settings,
    documentation: Documentation,
}

impl UiComponent for App {
    fn update(&mut self, message: Message) {
        self.working_space.update(message);
    }

    fn view(&self) -> Element<Message> {
        let display_focus = match self.focus {
            OnWindow::WorkingSpace => self.working_space.view(),
            OnWindow::Documentation => self.documentation.view(),
            OnWindow::Settings => self.settings.view(),
        };
        column![display_focus]
            .into()
    }
}

impl App {
    pub(crate) fn title(&self) -> String {
        format!("Slide Displacement Analysis - {}", self.project.title)
    }
}

#[derive(Default, Debug)]
pub(crate) struct WorkingSpace {
    menus: Menus,
    explorer: Explorer,
    viewer: Viewer,
}

impl UiComponent for WorkingSpace {
    fn update(&mut self, message: Message) {
        self.menus.update(message);
        self.explorer.update(message);
        self.viewer.update(message);
    }

    fn view(&self) -> Element<Message> {
        column![
            self.menus.view(),
            row![self.explorer.view(), self.viewer.view()]
        ].into()
    }
}
