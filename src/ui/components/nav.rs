use iced::{
    widget::{container, horizontal_space, row, Space},
    Command,
};

use super::{assets, style};

pub struct State {}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    CollapseSidebar,
    CheckUpdates,
}

impl State {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::CollapseSidebar => Command::none(),
            Event::CheckUpdates => Command::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        let content = container(row![
            assets::action(
                assets::menu_icon(),
                "Collapse",
                Some(Event::CollapseSidebar)
            ),
            horizontal_space(),
            assets::action(
                assets::update_icon(),
                "Check for updates",
                Some(Event::CheckUpdates)
            ),
            Space::with_width(10),
        ])
        .style(style::dynamic_colour);

        content.into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
