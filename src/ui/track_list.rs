use std::collections::HashMap;

use super::components::icons::{action, edit_icon, play_icon};
use crate::core::format::format_duration;
use crate::core::sql;

use iced::advanced::graphics::futures::event;
use iced::event::Event as IcedEvent;
use iced::keyboard;
use iced::keyboard::key;
use iced::widget::{
    self, button, center, column, container, horizontal_space, mouse_area, opaque, row, scrollable,
    stack, text, text_input,
};
use iced::Subscription;
use iced::{Alignment, Color, Command, Element, Length};

pub struct State {
    track_list: Vec<HashMap<String, String>>,
    show_modal: bool,
    new_display_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    PlayTrack(String, String, u64),
    Refresh,

    ShowModal(String),
    HideModal,
    NewDisplayName(String),
    Submit,
    DeleteTrack,
    KeyboardEvent(IcedEvent),
}

impl State {
    fn new() -> Self {
        Self {
            track_list: sql::get_all_music(),
            show_modal: false,
            new_display_name: String::new(),
        }
    }

    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::PlayTrack(video_id, _display_name, _duration) => {
                let data = sql::get_music(video_id);

                let info = data.get("display_name");

                Command::none()
            }
            Event::Refresh => {
                self.track_list = sql::get_all_music();

                Command::none()
            }
            Event::ShowModal(video_id) => {
                self.show_modal = true;
                widget::focus_next()
            }
            Event::HideModal => {
                self.hide_modal();

                Command::none()
            }
            Event::NewDisplayName(value) => {
                self.new_display_name = value;

                Command::none()
            }
            Event::Submit => {
                if !self.new_display_name.is_empty() {
                    self.hide_modal()
                }
                // TODO: Save functions
                Command::none()
            }
            Event::DeleteTrack => Command::none(),
            Event::KeyboardEvent(event) => match event {
                IcedEvent::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Tab),
                    modifiers,
                    ..
                }) => {
                    if modifiers.shift() {
                        widget::focus_previous()
                    } else {
                        widget::focus_next()
                    }
                }
                IcedEvent::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Escape),
                    ..
                }) => {
                    self.hide_modal();
                    Command::none()
                }
                _ => Command::none(),
            },
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        let mut column = column![row![
            text("Your Music").size(18),
            button("Refresh").on_press(Event::Refresh)
        ]
        .align_items(Alignment::Center)
        .spacing(20)]
        .spacing(10);

        for audio_file in &self.track_list {
            let video_id = audio_file.get("video_id").unwrap();
            let display_name = audio_file.get("display_name").unwrap();
            let duration = audio_file.get("duration").unwrap();
            let formatted_duration = format_duration(duration.parse::<u64>().unwrap());

            let row = row![
                action(
                    play_icon(),
                    display_name,
                    Some(Event::PlayTrack(
                        video_id.clone(),
                        display_name.clone(),
                        duration.parse::<u64>().unwrap()
                    )),
                ),
                text(display_name.clone()),
                horizontal_space(),
                text(formatted_duration.clone()),
                action(
                    edit_icon(),
                    "Edit",
                    Some(Event::ShowModal(video_id.clone()))
                ),
            ]
            .spacing(10)
            .align_items(Alignment::Start);

            column = column.push(row);
        }

        let content = container(
            scrollable(
                column![column]
                    .spacing(40)
                    .align_items(Alignment::Start)
                    .width(Length::Fill),
            )
            .height(Length::Fill),
        )
        .padding(10);

        if self.show_modal {
            let edit = container(
                column![
                    text("Edit Track").size(24),
                    column![
                        text("New Track Name:"),
                        text_input("Enter here...", &self.new_display_name)
                            .on_input(Event::NewDisplayName),
                    ]
                    .align_items(Alignment::Center)
                    .padding(10),
                    button("Delete Track")
                        .style(button::danger)
                        .on_press(Event::DeleteTrack),
                ]
                .align_items(Alignment::Center)
                .spacing(20),
            )
            .style(container::rounded_box)
            .width(300);

            modal(content, edit, Event::HideModal)
        } else {
            content.into()
        }
    }

    pub fn subscription(&self) -> Subscription<Event> {
        event::listen().map(Event::KeyboardEvent)
    }

    fn hide_modal(&mut self) {
        self.show_modal = false;
        self.new_display_name.clear();
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        mouse_area(center(opaque(content)).style(|_theme| {
            container::Style {
                background: Some(
                    Color {
                        a: 0.8,
                        ..Color::BLACK
                    }
                    .into(),
                ),
                ..container::Style::default()
            }
        }))
        .on_press(on_blur)
    ]
    .into()
}
