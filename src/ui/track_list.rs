use std::collections::HashMap;

use super::components::assets;
use crate::core::format;
use crate::core::request;
use crate::core::sql;

use iced::advanced::graphics::futures::event;
use iced::event::Event as IcedEvent;
use iced::keyboard;
use iced::keyboard::key;
use iced::widget::{
    self, button, column, container, horizontal_space, row, scrollable, text, text_input, Space,
};
use iced::Subscription;
use iced::{Alignment, Command, Element, Length};

use log::info;

pub struct State {
    track_list: Vec<HashMap<String, String>>,
    show_modal: bool,
    new_display_name: String,
    active_video_id: Option<String>,
    active_display_name: Option<String>,
    thumbnails_received: bool,
    thumbnails: Vec<HashMap<String, iced::advanced::image::Handle>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    HideModal,
    Submit,
    DeleteTrack,
    GetThumbnailHandles,
    ThumbnailsReceived(Vec<HashMap<String, iced::advanced::image::Handle>>),
    NewDisplayName(String),
    ShowModal(String, String),
    PlayTrack(String, String, u64),
    KeyboardEvent(IcedEvent),
}

impl State {
    fn new() -> Self {
        Self {
            track_list: sql::get_all_music(),
            show_modal: false,
            new_display_name: String::new(),
            active_video_id: None,
            active_display_name: None,
            thumbnails_received: false,
            thumbnails: Vec::new(),
        }
    }

    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::GetThumbnailHandles => {
                self.thumbnails_received = false;

                self.track_list = sql::get_all_music();

                let video_ids: Vec<String> = self
                    .track_list
                    .iter()
                    .map(|track| track.get("video_id").unwrap().clone())
                    .collect();

                Command::perform(
                    request::request_thumbnails(video_ids),
                    Event::ThumbnailsReceived,
                )
            }
            Event::ThumbnailsReceived(thumbnails) => {
                self.thumbnails = thumbnails;

                self.thumbnails_received = true;

                Command::none()
            }

            Event::PlayTrack(_video_id, _display_name, _duration) => Command::none(),

            Event::ShowModal(video_id, display_name) => {
                info!("Showing modal for track with video_id: {}", video_id);

                self.show_modal = true;
                self.active_video_id = Some(video_id);
                self.active_display_name = Some(display_name.clone());
                self.new_display_name = display_name;
                widget::focus_next()
            }
            Event::HideModal => {
                info!("Hiding modal.");
                self.hide_modal();

                Command::none()
            }
            Event::NewDisplayName(value) => {
                self.new_display_name = value;

                Command::none()
            }
            Event::Submit => {
                let active = self.active_video_id.clone().unwrap();
                let new_display_name = self.new_display_name.clone();

                self.hide_modal();

                let _ = sql::edit_display_name(active, new_display_name);

                Command::none()
            }
            Event::DeleteTrack => {
                let active = self.active_video_id.clone().unwrap();
                sql::delete_music(active).unwrap();

                self.hide_modal();

                self.active_video_id = None;
                Command::none()
            }
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
                    info!("Hiding modal via escape key.");

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
            button("Refresh").on_press(Event::GetThumbnailHandles)
        ]];

        for audio_file in &self.track_list {
            let video_id = audio_file.get("video_id").unwrap();
            let display_name = audio_file.get("display_name").unwrap();
            let duration = audio_file.get("duration").unwrap();
            let formatted_duration = format::format_duration(duration.parse::<u64>().unwrap());

            let row: Element<Event>;

            if self.thumbnails_received {
                let thumbnail_handle = self
                    .thumbnails
                    .iter()
                    .find(|thumbnail| thumbnail.contains_key(video_id))
                    .unwrap()
                    .get(video_id)
                    .unwrap();

                row = row![
                    assets::action(
                        assets::play_icon(),
                        display_name,
                        Some(Event::PlayTrack(
                            video_id.clone(),
                            display_name.clone(),
                            duration.parse::<u64>().unwrap()
                        )),
                    ),
                    assets::thumbnail(thumbnail_handle.clone()),
                    Space::with_width(10),
                    text(display_name.clone()),
                    horizontal_space(),
                    text(formatted_duration.clone()),
                    Space::with_width(10),
                    assets::action(
                        assets::edit_icon(),
                        "Edit",
                        Some(Event::ShowModal(video_id.clone(), display_name.clone()))
                    ),
                    Space::with_width(30),
                ]
                .align_items(Alignment::Center)
                .spacing(10)
                .into();
            } else {
                row = row![
                    assets::action(
                        assets::play_icon(),
                        display_name,
                        Some(Event::PlayTrack(
                            video_id.clone(),
                            display_name.clone(),
                            duration.parse::<u64>().unwrap()
                        )),
                    ),
                    text("..."),
                    Space::with_width(10),
                    text(display_name.clone()),
                    horizontal_space(),
                    text(formatted_duration.clone()),
                    Space::with_width(10),
                    assets::action(
                        assets::edit_icon(),
                        "Edit",
                        Some(Event::ShowModal(video_id.clone(), display_name.clone()))
                    ),
                    Space::with_width(30),
                ]
                .align_items(Alignment::Center)
                .spacing(10)
                .into();
            }

            column = column.push(row);
        }

        let content = container(
            scrollable(column.spacing(5))
                .height(Length::Fill)
                .width(Length::Fill),
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
                    .spacing(10),
                    row![
                        button("Delete Track")
                            .style(button::danger)
                            .on_press(Event::DeleteTrack),
                        button("Submit")
                            .style(button::success)
                            .on_press(Event::Submit),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center),
                ]
                .align_items(Alignment::Center)
                .spacing(20),
            )
            .style(container::rounded_box)
            .width(300);

            assets::modal(content, edit, Event::HideModal)
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
