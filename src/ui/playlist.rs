use std::collections::HashMap;

use super::components::style;
use crate::core::format;

use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Length, Task};

use crate::core::db;

pub struct State {
    create_playlist_mode: bool,
    playlist_view: bool,
    playlist_name_input: String,
    playlists: Vec<HashMap<String, String>>,
    tracks: Vec<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    OpenInListMode,
    OpenInCreateMode,
    CreatePlaylist,
    OpenPlaylist(i32),
    PlaylistNameInput(String),
}

impl State {
    pub fn update(&mut self, message: Event) -> Task<Event> {
        match message {
            Event::OpenPlaylist(index) => {
                self.playlist_view = true;

                self.tracks = db::get_playlist_tracks(
                    self.playlists[index as usize]
                        .get("id")
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                );

                Task::none()
            }
            Event::OpenInListMode => {
                self.playlist_view = false;
                self.create_playlist_mode = false;

                Task::none()
            }

            Event::CreatePlaylist => {
                self.playlist_view = false;
                self.create_playlist_mode = false;

                let _ = db::add_playlist(self.playlist_name_input.clone());

                self.playlists = db::get_all_playlists();

                Task::none()
            }
            Event::OpenInCreateMode => {
                self.create_playlist_mode = true;

                Task::none()
            }

            Event::PlaylistNameInput(value) => {
                self.playlist_name_input = value;

                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        if self.playlist_view {
            let mut col = column![];

            for (index, track) in self.tracks.iter().enumerate() {
                match db::get_music_from_id(track.get("music_id").unwrap().parse::<i32>().unwrap())
                {
                    Ok(music) => {
                        col = col.push(
                            button(text(format::trunc_name(
                                music.get("display_name").unwrap().clone().as_str(),
                            )))
                            .on_press(Event::OpenPlaylist(index as i32))
                            .style(style::sidebar_button),
                        );
                    }
                    Err(_) => {
                        col = col.push(
                            button(text("[error getting this track]"))
                                .on_press(Event::OpenPlaylist(index as i32))
                                .style(style::sidebar_button),
                        );
                    }
                }
            }

            let content = container(scrollable(
                col.spacing(40)
                    .align_items(Alignment::Start)
                    .width(Length::Fill),
            ))
            .padding(10);

            return content.into();
        }

        if self.create_playlist_mode {
            return container(
                column![
                    text("Create new playlist").size(22),
                    row![
                        text("Name:"),
                        text_input("...", &self.playlist_name_input)
                            .on_input(Event::PlaylistNameInput)
                    ]
                    .spacing(10)
                    .width(400)
                    .align_items(Alignment::Center),
                    button("Create new playlist").on_press(Event::CreatePlaylist),
                ]
                .spacing(15)
                .align_items(Alignment::Center),
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
        }

        let mut col = column![];

        // amend for loop to get an index
        for (index, playlist) in self.playlists.iter().enumerate() {
            col = col.push(
                button(text(playlist.get("name").unwrap()))
                    .on_press(Event::OpenPlaylist(index as i32))
                    .style(style::sidebar_button),
            );
        }

        let content = container(
            column![
                text("Your Playlists").size(22),
                scrollable(col.spacing(25).align_items(Alignment::Center))
            ]
            .align_items(Alignment::Center)
            .spacing(10),
        )
        .padding(10)
        .center_x(Length::Fill);

        content.into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            create_playlist_mode: false,
            playlist_view: false,
            playlist_name_input: String::new(),
            playlists: db::get_all_playlists(),
            tracks: Vec::new(),
        }
    }
}
