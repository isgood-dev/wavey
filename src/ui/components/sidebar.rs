use super::assets;
use super::style;

use iced::widget::{column, container, text, Space};
use iced::{Alignment, Command, Length};

pub struct State {}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    OpenTrackList,
    OpenSettings,
    OpenEdit,
    OpenDownload,
    CreatePlaylist,
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::CreatePlaylist => Command::none(),
            Event::OpenTrackList => Command::none(),
            Event::OpenSettings => Command::none(),
            Event::OpenEdit => Command::none(),
            Event::OpenDownload => Command::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        container(
            column![
                text("MY MUSIC").size(12).style(style::sidebar_text),
                assets::action_with_text(
                    assets::home_icon(),
                    "Home Page",
                    Some(Event::OpenTrackList)
                ),
                assets::action_with_text(assets::edit_icon(), "Edit Music", Some(Event::OpenEdit)),
                assets::action_with_text(
                    assets::download_icon(),
                    "Add Music",
                    Some(Event::OpenDownload)
                ),
                assets::action_with_text(
                    assets::settings_icon(),
                    "Settings",
                    Some(Event::OpenSettings)
                ),
                Space::with_height(10),
                text("MY PLAYLISTS").size(12).style(style::sidebar_text),
                assets::action_with_text(
                    assets::add_icon(),
                    "New Playlist",
                    Some(Event::CreatePlaylist)
                ),
            ]
            .spacing(10)
            .padding(10)
            .width(180)
            .align_items(Alignment::Center),
        )
        .style(style::dynamic_colour)
        .height(Length::Fill)
        .into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}
