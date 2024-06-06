use std::collections::HashMap;

use super::components::assets;
use crate::core::request;
use crate::core::youtube;

use iced::widget::{column, container, image, row, scrollable, text};
use iced::{Alignment, Command, Length};

pub struct State {
    loading: bool,
    results: Vec<HashMap<String, String>>,
    thumbnails: Vec<iced::advanced::image::Handle>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    PopulateResults(Vec<HashMap<String, String>>),
    ThumbnailReceived(Result<Vec<Vec<u8>>, request::RequestError>),
    DownloadPressed(String),
    DownloadComplete(Result<(), youtube::StatusError>),
}

impl State {
    pub fn new() -> Self {
        Self {
            loading: true,
            results: Vec::new(),
            thumbnails: Vec::new(),
        }
    }

    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::DownloadComplete(_status) => Command::none(),
            Event::DownloadPressed(url) => {
                Command::perform(youtube::download_from_url(url), Event::DownloadComplete)
            }
            Event::PopulateResults(data) => {
                self.results = data.clone();

                Command::perform(
                    request::request_all_thumbnails(data),
                    Event::ThumbnailReceived,
                )
            }

            Event::ThumbnailReceived(response) => {
                match response {
                    Ok(data) => {
                        for thumbnail in data {
                            let handle = iced::advanced::image::Handle::from_bytes(thumbnail);
                            self.thumbnails.push(handle);
                        }
                    }
                    Err(_) => {}
                };

                self.loading = false;

                Command::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        if self.loading {
            container(text("Loading...").size(18))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            let mut column = column![];

            for (index, result) in self.results.iter().enumerate() {
                let heading = format!(
                    "{} - {}",
                    result.get("title").unwrap(),
                    result.get("channel").unwrap()
                );
                let row = row![
                    assets::action(
                        assets::download_icon(),
                        "Download",
                        Some(Event::DownloadPressed(
                            result.get("video_id").unwrap().to_string()
                        ))
                    ),
                    image(self.thumbnails[index].clone()).width(130).height(100),
                    text(heading).size(16),
                ]
                .align_items(Alignment::Center)
                .spacing(10);

                column = column.push(row);
            }

            let content = container(column![
                text("Search Results").size(18),
                scrollable(
                    column
                        .spacing(20)
                        .align_items(Alignment::Start)
                        .width(Length::Fill),
                )
                .height(Length::Fill),
            ])
            .padding(10);

            content.into()
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
