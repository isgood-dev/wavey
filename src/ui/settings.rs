use super::components::theme;
use crate::core::json;

use iced::widget::{button, column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Length, Task};

pub struct State {
    themes: theme::Themes,
    rpc_enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    ThemeSelected(theme::Themes),
    ToggleRpcEnabled,
}

impl State {
    pub fn update(&mut self, message: Event) -> Task<Event> {
        match message {
            Event::ThemeSelected(theme) => {
                self.themes = theme;

                json::set_theme(theme.to_string().as_str()).unwrap();

                Task::none()
            }

            Event::ToggleRpcEnabled => {
                if self.rpc_enabled {
                    self.rpc_enabled = false;
                } else {
                    self.rpc_enabled = true;
                }

                json::set_rpc_enabled(self.rpc_enabled).unwrap();

                Task::none()
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
                        pick_list(theme::Themes::ALL, Some(self.themes), Event::ThemeSelected),
                        text("Themes beside Light and Dark are experimental.").size(14),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(10),
                    row![
                        text("Discord Rich Presence:"),
                        button(if self.rpc_enabled {
                            "Enabled"
                        } else {
                            "Disabled"
                        })
                        .on_press(Event::ToggleRpcEnabled)
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
            themes: theme::Themes::default(),
            rpc_enabled: json::get_rpc_enabled().unwrap(),
        }
    }
}
