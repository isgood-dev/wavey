use std::collections::HashMap;

use iced::advanced::image::Bytes;
use iced::{
    widget::{button, column, container, image, row, scrollable, text, Container},
    Alignment, Command, Length,
};
use reqwest::Client;
// use bytes::Bytes;

pub struct State {
    loading: bool,
    results: Vec<HashMap<String, String>>,
    thumbnails: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    PopulateResults(Vec<HashMap<String, String>>),
    ThumbnailReceived(Vec<Vec<u8>>),
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
            Event::PopulateResults(data) => {
                println!("1");
                self.results = data.clone();

                Command::perform(request_all_thumbnails(data), Event::ThumbnailReceived)
            }

            Event::ThumbnailReceived(data) => {
                println!("2");
                self.thumbnails = data;
                println!("{}", self.thumbnails.len());
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
            let mut column = column![text("Please select what to download").size(18)].spacing(10);

            for (index, result) in self.results.iter().enumerate() {
                let row = row![
                    thumbnail(self.thumbnails[index].clone()), // Clone the value here
                    column![
                        text(result.get("title").unwrap()),
                        text(result.get("url").unwrap()),
                        text(result.get("channel").unwrap()),
                    ],
                    button("Download"),
                ]
                .align_items(Alignment::Center)
                .spacing(10);

                column = column.push(row);
            }

            let content = container(
                scrollable(
                    column
                        .spacing(40)
                        .align_items(Alignment::Center)
                        .width(Length::Fill),
                )
                .height(Length::Fill),
            )
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
    container(image(handle).width(100).height(70)).center_x()
}

async fn request_thumbnail(url: String) -> Result<Bytes, reqwest::Error> {
    println!("Requesting thumbnail: {}", url);
    let client = Client::new();

    let response = client.get(&url).send().await?;

    println!("Response: {:?}", response.status());

    let bytes = response.bytes().await?;

    Ok(bytes)
}

async fn request_all_thumbnails(results: Vec<HashMap<String, String>>) -> Vec<Vec<u8>> {
    println!("3");
    let mut thumbnails = Vec::new();

    for result in results {
        println!("Requesting thumbnail: {}", result.get("thumbnail").unwrap());
        let url = result.get("thumbnail").unwrap().clone();

        let bytes = request_thumbnail(url).await.unwrap();

        thumbnails.push(bytes.to_vec());
    }

    thumbnails
}
