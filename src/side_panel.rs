use iced::widget::{button, column, row, text, Column, Button};
use iced::{Length, Alignment, theme};
use crate::app::*;

#[derive(Debug, Clone, Copy)]
pub(crate) enum MessageSidePanel {
    TabSelected(Tab),
}

#[derive(Default, Debug)]
pub(crate) struct SidePanel {
    active_tab: Tab
}

impl SidePanel {
    pub(crate) fn update(&mut self, message: MessageSidePanel, handler: &mut ProjectHandler) {
        match message {
            MessageSidePanel::TabSelected(tab) => self.active_tab = tab,
        }
    }

    pub(crate) fn view(&self, handler: &ProjectHandler) -> iced::Element<Message> {
        let tab = self.active_tab.view_tab();
        let content = self.active_tab.view_content(handler);
        row![tab, content].into()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(crate) enum Tab {
    Settings,
    #[default]
    Explorer,
    Command,
    Documentation,
}

impl Tab {
    fn view_content(&self, handler: &ProjectHandler) -> iced::Element<Message> {
        let tab_bar = column![];

        let content = match self {
            Tab::Settings => {
                text("Settings")
            },
            Tab::Explorer => {
                text("Explorer")
            },
            Tab::Command => {
                text("Command")
            },
            Tab::Documentation => {
                text("Documentation")
            },
        };

        row![tab_bar, content].into()
    }

    fn view_tab(&self) -> iced::Element<Message> {
        let setting_icon = tab_button("S", Tab::Settings, *self);
        let explorer_icon = tab_button("E", Tab::Explorer, *self);
        let command_icon = tab_button("C", Tab::Command, *self);
        let docs_icon = tab_button("D", Tab::Documentation, *self);

        column![setting_icon, explorer_icon, command_icon, docs_icon].into()
    }
}

fn tab_button(name: &str, tab: Tab, active_tab: Tab) -> Button<Message> {
    let button = button(name);
    if tab == active_tab {
        println!("here")
    }
    button
        .on_press(Message::SidePanel(MessageSidePanel::TabSelected(tab)))
        .height(Length::Fixed(150.))
        .padding(5)
}
