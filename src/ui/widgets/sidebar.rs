use super::icons::{action_with_text, download_icon, edit_icon, home_icon, settings_icon};

use iced::{
    widget::{column, container},
    Alignment, Color, Command, Length,
};

pub struct State {}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    OpenTrackList,
    OpenSettings,
    OpenEdit,
    OpenDownload,
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::OpenTrackList => Command::none(),
            Event::OpenSettings => Command::none(),
            Event::OpenEdit => Command::none(),
            Event::OpenDownload => Command::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        container(
            column![
                action_with_text(home_icon(), "Home Page", Some(Event::OpenTrackList)),
                action_with_text(edit_icon(), "Edit Music", Some(Event::OpenEdit)),
                action_with_text(download_icon(), "Add Music", Some(Event::OpenDownload)),
                action_with_text(settings_icon(), "Settings", Some(Event::OpenSettings)),
            ]
            .spacing(20)
            .padding(10)
            .width(200)
            .align_items(Alignment::Center),
        )
        .style(|_theme| {
            container::Style::default()
                .with_background(Color::from_rgba(0.102, 0.102, 0.102, 1.0))
                .with_border(Color::from_rgb(255.0, 0.0, 0.0), 0)
        })
        .height(Length::Fill)
        .into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}
