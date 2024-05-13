use std::collections::HashMap;

use crate::core::youtube::download_from_url;

use super::components::icons::{action, download_icon};

use iced::advanced::image::Bytes;
use iced::{
    widget::{column, container, image, row, scrollable, text, Container},
    Alignment, Command, Length,
};
use reqwest::Client;

pub struct State {
    loading: bool,
    results: Vec<HashMap<String, String>>,
    thumbnails: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    PopulateResults(Vec<HashMap<String, String>>),
    ThumbnailReceived(Vec<Vec<u8>>),
    DownloadPressed(String),
    DownloadComplete(bool),
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
            Event::DownloadPressed(url) => Command::perform(download_from_url(url), Event::DownloadComplete),
            Event::PopulateResults(data) => {
                self.results = data.clone();

                Command::perform(request_all_thumbnails(data), Event::ThumbnailReceived)
            }

            Event::ThumbnailReceived(data) => {
                self.thumbnails = data;
                self.loading = false;

                Command::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        if self.loading {
            container(text("Loading...").size(18))
                .center_x()
                .center_y()
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
                    action(
                        download_icon(),
                        "Download",
                        Some(Event::DownloadPressed(
                            result.get("video_id").unwrap().to_string()
                        ))
                    ),
                    thumbnail(self.thumbnails[index].clone())
                        .width(150)
                        .max_width(150), // Clone the value here
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

fn thumbnail<'a>(url: Vec<u8>) -> Container<'a, Event> {
    let handle = image::Handle::from_bytes(Bytes::from(url));
    container(image(handle).width(120).height(90)).center_x()
}

async fn request_thumbnail(url: String) -> Result<Bytes, reqwest::Error> {
    let client = Client::new();

    let response = client.get(&url).send().await?;

    let bytes = response.bytes().await?;

    Ok(bytes)
}

async fn request_all_thumbnails(results: Vec<HashMap<String, String>>) -> Vec<Vec<u8>> {
    let mut thumbnails = Vec::new();

    for result in results {
        let url = result.get("thumbnail").unwrap().clone();

        let bytes = request_thumbnail(url).await.unwrap();

        thumbnails.push(bytes.to_vec());
    }

    thumbnails
}
