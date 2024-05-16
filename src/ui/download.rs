use std::collections::HashMap;

use iced::{
    widget::{button, column, container, scrollable, text, text_input},
    Alignment, Command, Length,
};

use crate::core::youtube::{get_search_results, YouTubeError};

use super::super::core::youtube::download_from_url;

pub struct State {
    yt_url: String,
    title: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    YouTubeURLInput(String),
    SongNameInput(String),
    DownloadQueryReceived(Result<Vec<HashMap<String, String>>, YouTubeError>),
    Download,
    Search,
    DownloadResult(bool),
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::DownloadQueryReceived(Ok(_data)) => Command::none(),
            Event::DownloadQueryReceived(Err(_)) => Command::none(),
            Event::Search => {
                let query = self.title.clone();

                Command::perform(get_search_results(query), Event::DownloadQueryReceived)
            }
            Event::SongNameInput(value) => {
                self.title = value;

                Command::none()
            }

            Event::YouTubeURLInput(value) => {
                self.yt_url = value;

                Command::none()
            }

            Event::Download => {
                let yt_url = self.yt_url.clone();

                Command::perform(download_from_url(yt_url), Event::DownloadResult)
            }

            Event::DownloadResult(status) => {
                if status {
                    print!("Success");
                } else {
                    print!("Not successful");
                }

                Command::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        let content = container(
            scrollable(
                column![
                    text("Download/Import Music").size(18),
                    text_input("Youtube URL...", &self.yt_url).on_input(Event::YouTubeURLInput),
                    button("Download").on_press(Event::Download),
                    text_input("Song Name...", &self.title).on_input(Event::SongNameInput),
                    button("Search").on_press(Event::Search),
                    
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
        Self {
            yt_url: String::new(),
            title: String::new(),
        }
    }
}
