use super::super::core::audio_dir::get_audio_files;
use super::widgets::icons::{action, play_icon};

use iced::{
    widget::{column, container, row, scrollable, text},
    Alignment, Command, Length,
};

pub struct State {
    audio_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    PlaySongPressed,
}

impl State {
    fn new() -> Self {
        Self {
            audio_files: get_audio_files(),
        }
    }

    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::PlaySongPressed => {
                println!("Play song pressed");
                Command::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        let mut column = column![].spacing(10);

        for file in &self.audio_files {
            let action = action(play_icon(), "Play", Some(Event::PlaySongPressed));
            let text = text(file);

            column = column.push(row![action, text].spacing(10).align_items(Alignment::Center));
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
