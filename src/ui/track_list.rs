use iced::{Alignment, Command, Length, Theme};
use iced::widget::{button, column, container, horizontal_space, row, scrollable, text};

use super::pages::{Page, Pages};

pub struct State {
    counter: i32,
}

#[derive(Debug, Clone)]
pub enum Event {
    DownloadPagePressed,
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::DownloadPagePressed => {
                Pages

                Command::none()
            },
        }

    }

    pub fn view(&self) -> iced::Element<Event> {
        let header = container(
            row![
                text("Music Player"),
                horizontal_space(),
                "Test",
            ]
            .padding(10)
            .align_items(Alignment::Center),
        )
        .style(|theme: &Theme| {
            let pallette = theme.extended_palette();

            container::Style::default()
                .with_border(pallette.background.strong.color, 1)
        });

        let sidebar = container(
            column!["Sidebar", button("Test Button 1"), button("Test Button 2")]
                .spacing(10)
                .padding(10)
                .width(200)
                .align_items(Alignment::Center),
        )
        .style(container::rounded_box)
        .height(Length::Fill);

        let content = container(
            scrollable(
                column![text("abc"), button("content")]
                .spacing(40)
                .align_items(Alignment::Start)
                .width(Length::Fill),
            )
            .height(Length::Fill)
        )
        .padding(10);

        container(
            column!(header, row![sidebar, content])
        )
        .into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self { counter: 0 }
    }
}