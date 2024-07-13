use std::collections::HashMap;

use super::helpers;
use super::style;
use crate::core::db;

use iced::widget::{button, column, container, scrollable, text, Space};
use iced::{Alignment, Length, Task};

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
    pub fn update(&mut self, message: Event) -> Task<Event> {
        match message {
            Event::CollapseToggle => {
                self.collapsed = !self.collapsed;

                Task::none()
            }
            Event::OpenPlaylist(_value) => Task::none(),

            Event::UpdatePlaylists => {
                self.playlists = db::get_all_playlists();

                Task::none()
            }
            Event::CreatePlaylist => Task::none(),
            Event::OpenTrackList => Task::none(),
            Event::OpenSettings => Task::none(),
            Event::OpenPlaylists => Task::none(),
            Event::OpenDownload => Task::none(),
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
                .align_x(Alignment::Center),
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
                .align_x(Alignment::Center),
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
