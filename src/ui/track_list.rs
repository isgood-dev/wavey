use std::collections::HashMap;

use crate::core::{format::format_duration, sql};

use super::components::icons::{action, edit_icon, play_icon};

use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text},
    Alignment, Command, Length,
};

pub struct State {
    track_list: Vec<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    PlayTrack(String),
    EditTrack(String),
    Refresh,
}

impl State {
    fn new() -> Self {
        Self {
            track_list: sql::get_all_music(),
        }
    }

    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::EditTrack(video_id) => {
                println!("Editing: {}", video_id);

                Command::none()
            }
            Event::PlayTrack(video_id) => {
                let data = sql::get_music(video_id);

                let info = data.get("display_name");

                println!("Playing: {}", info.unwrap());

                Command::none()
            }
            Event::Refresh => {
                self.track_list = sql::get_all_music();

                Command::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        let mut column = column![row![text("Your Music").size(18), button("Refresh").on_press(Event::Refresh)].align_items(Alignment::Center).spacing(20)].spacing(10);

        for audio_file in &self.track_list {
            let video_id = audio_file.get("video_id").unwrap();
            let display_name = audio_file.get("display_name").unwrap();
            let duration = audio_file.get("duration").unwrap();
            let formatted_duration = format_duration(duration.parse::<u64>().unwrap());

            let row = row![
                action(
                    play_icon(),
                    display_name,
                    Some(Event::PlayTrack(video_id.clone())),
                ),
                action(
                    edit_icon(),
                    "Edit",
                    Some(Event::EditTrack(video_id.clone()))
                ),
                text(display_name.clone()),
                horizontal_space(),
                text(formatted_duration.clone()),
            ]
            .spacing(10)
            .align_items(Alignment::Start);

            column = column.push(row);
        }

        let content = container(
            scrollable(
                column![column]
                    .spacing(40)
                    .align_items(Alignment::Start)
                    .width(Length::Fill),
            )
            .height(Length::Fill),
        )
        .padding(10);

        content.into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
