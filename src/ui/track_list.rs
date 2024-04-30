use super::super::core::audio_dir::get_audio_files;

use iced::{
    widget::{button, column, container, row, scrollable, text},
    Alignment, Command, Length,
};

pub struct State {
    audio_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {

}

impl State {
    fn new() -> Self {
        Self {
            audio_files: get_audio_files(),
        }
    }

    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {

        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        let content = container(
            scrollable(
                column![
                    "Track list",
                    row![button("Test Button"), text("Test")]
                ]
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
