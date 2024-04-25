use iced::{Alignment, Command, Length, Theme, Element};
use iced::widget::{button, column, container, horizontal_space, row, scrollable, text, text_input};

pub struct State {
    youtube_url: String,
}

#[derive(Debug, Clone)]
pub enum Event {

}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {

        }
    }

    pub fn view(&self) -> Element<Event> {
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
                column![
                    text("Download/Import audio").size(16),
                    text("Download new songs or import ones from your device."),
                    text_input("Enter YouTube URL here...", &self.youtube_url),
                    button("content")
                ]
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
        Self { 
            youtube_url: String::new(),
        }
    }
}