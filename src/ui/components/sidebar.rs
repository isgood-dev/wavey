use super::icons::{action_with_text, download_icon, edit_icon, home_icon, settings_icon};
use super::style::dynamic_colour;

use iced::widget::{column, container};
use iced::{Alignment, Command, Length};

pub struct State {}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    OpenTrackList,
    OpenSettings,
    OpenEdit,
    OpenDownload,
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::OpenTrackList => Command::none(),
            Event::OpenSettings => Command::none(),
            Event::OpenEdit => Command::none(),
            Event::OpenDownload => Command::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        container(
            column![
                action_with_text(home_icon(), "Home Page", Some(Event::OpenTrackList)),
                action_with_text(edit_icon(), "Edit Music", Some(Event::OpenEdit)),
                action_with_text(download_icon(), "Add Music", Some(Event::OpenDownload)),
                action_with_text(settings_icon(), "Settings", Some(Event::OpenSettings)),
            ]
            .spacing(20)
            .padding(10)
            .width(200)
            .align_items(Alignment::Center),
        )
        .style(dynamic_colour)
        .height(Length::Fill)
        .into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}
