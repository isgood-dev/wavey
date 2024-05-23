use super::components::theme::Themes;
use crate::core::json::set_theme;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Command, Length};

pub struct State {
    themes: Themes,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    ThemeSelected(Themes),
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::ThemeSelected(theme) => {
                self.themes = theme;

                set_theme(theme.to_string().as_str()).unwrap();

                Command::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        let content = container(
            scrollable(
                column![
                    text("Settings").size(18),
                    row![
                        text("Theme:"),
                        pick_list(Themes::ALL, Some(self.themes), Event::ThemeSelected),
                        text("Themes beside Light and Dark are experimental.").size(14),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(10),
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
            themes: Themes::default(),
        }
    }
}
