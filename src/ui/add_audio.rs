use iced::{Alignment, Command, Length, Theme, Element};
use iced::widget::{button, column, container, horizontal_space, row, text};

pub struct State {
    counter: i32,
}

#[derive(Debug, Clone)]
pub enum Event {
    Increment,
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::Increment => {
                self.counter += 1;

                Command::none()
            }
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

        column![header, row![sidebar]].into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self { counter: 0 }
    }
}