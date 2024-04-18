use iced::{Alignment, Element, Length, Theme};
use iced::widget::{button, column, container, horizontal_space, row, scrollable, text};

pub fn main() -> iced::Result {
    iced::program("Music Player", MusicPlayer::update, MusicPlayer::view)
        .window_size((900.0, 700.0))
        .run()
}

enum Page {
    StartPage,
}
struct MusicPlayer {
    current_page: Page,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl MusicPlayer {
    fn new() -> Self {
        Self {
            current_page: Page::StartPage,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {}
    }

    pub fn view(&self) -> Element<Message> {
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
                .spacing(40)
                .padding(10)
                .width(200)
                .align_items(Alignment::Center),
        )
        .style(container::rounded_box)
        .height(Length::Fill);

        let content = container(
            scrollable(
                column![
                    "Downloaded Songs",
                    row![button("Test Button"), text("Test")]
                ]
                .spacing(40)
                .align_items(Alignment::Start)
                .width(Length::Fill),
            )
            .height(Length::Fill)
        )
        .padding(10);


        column![header, row![sidebar, content]].into()
    }
}

impl Default for MusicPlayer {
    fn default() -> Self {
        Self::new()
    }
}
