use std::collections::HashMap;

use crate::core::db;

use super::helpers;
use super::style;

use iced::widget::{button, column, container, scrollable, text, Space};
use iced::{Alignment, Command, Length};

pub struct State {
    playlists: Vec<HashMap<String, String>>,
    collapsed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    CollapseToggle,
    OpenTrackList,
    OpenSettings,
    OpenPlaylists,
    OpenDownload,
    CreatePlaylist,
    UpdatePlaylists,
    OpenPlaylist(i32),
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::CollapseToggle => {
                self.collapsed = !self.collapsed;

                Command::none()
            }
            Event::OpenPlaylist(_value) => Command::none(),

            Event::UpdatePlaylists => {
                self.playlists = db::get_all_playlists();

                Command::none()
            }
            Event::CreatePlaylist => Command::none(),
            Event::OpenTrackList => Command::none(),
            Event::OpenSettings => Command::none(),
            Event::OpenPlaylists => Command::none(),
            Event::OpenDownload => Command::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        let mut col = column![];

        let mut i = 0;

        for playlist in &self.playlists {
            col = col.push(
                button(text(playlist.get("name").unwrap()))
                    .style(style::sidebar_button)
                    .on_press(Event::OpenPlaylist(i)),
            );

            i += 1;
        }

        if self.collapsed {
            container(
                column![
                    helpers::action(
                        helpers::home_icon(),
                        "Home Page",
                        Some(Event::OpenTrackList)
                    ),
                    helpers::action(
                        helpers::list_icon(),
                        "Playlists",
                        Some(Event::OpenPlaylists)
                    ),
                    helpers::action(
                        helpers::download_icon(),
                        "Add Music",
                        Some(Event::OpenDownload)
                    ),
                    helpers::action(
                        helpers::settings_icon(),
                        "Settings",
                        Some(Event::OpenSettings)
                    ),
                ]
                .spacing(10)
                .padding(10)
                .width(50)
                .align_items(Alignment::Center),
            )
            .style(style::dynamic_colour)
            .height(Length::Fill)
            .into()
        } else {
            container(
                column![
                    text("MY MUSIC").size(12).style(style::sidebar_text),
                    helpers::action_with_text(
                        helpers::home_icon(),
                        "Home Page",
                        Some(Event::OpenTrackList)
                    ),
                    helpers::action_with_text(
                        helpers::list_icon(),
                        "Playlists",
                        Some(Event::OpenPlaylists)
                    ),
                    helpers::action_with_text(
                        helpers::download_icon(),
                        "Add Music",
                        Some(Event::OpenDownload)
                    ),
                    helpers::action_with_text(
                        helpers::settings_icon(),
                        "Settings",
                        Some(Event::OpenSettings)
                    ),
                    Space::with_height(10),
                    text("MY PLAYLISTS").size(12).style(style::sidebar_text),
                    helpers::action_with_text(
                        helpers::add_icon(),
                        "New Playlist",
                        Some(Event::CreatePlaylist)
                    ),
                    scrollable(col.spacing(5)).width(Length::Fill),
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
}

impl Default for State {
    fn default() -> Self {
        Self {
            collapsed: false,
            playlists: db::get_all_playlists(),
        }
    }
}
