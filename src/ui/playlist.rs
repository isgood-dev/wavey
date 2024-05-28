use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Command, Length};

use crate::core::sql;
pub struct State {
    create_playlist_mode: bool,
    playlist_name_input: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    OpenInCreateMode,
    CreatePlaylist,
    PlaylistNameInput(String),
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::CreatePlaylist => {
                self.create_playlist_mode = false;

                let _ = sql::add_playlist(self.playlist_name_input.clone());

                Command::none()
            }
            Event::OpenInCreateMode => {
                self.create_playlist_mode = true;

                Command::none()
            }

            Event::PlaylistNameInput(value) => {
                self.playlist_name_input = value;

                Command::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
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
            .center_x()
            .center_y()
            .into();
        }

        let content = container(
            column![text("").size(20)]
                .spacing(40)
                .align_items(Alignment::Start)
                .width(Length::Fill),
        )
        .padding(10);

        content.into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            create_playlist_mode: false,
            playlist_name_input: String::new(),
        }
    }
}
