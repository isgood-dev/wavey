use super::assets;
use super::style;
use crate::core::format;

use iced::widget::{column, container, row, slider, text};
use iced::{time, Alignment, Command, Element, Length};

use tokio::time::Duration;

pub struct State {
    formatted_current_duration: String,
    formatted_total_duration: String,
    total_duration: u64,
    slider_value: f32,
    seconds_passed: u64,
    slider_is_active: bool,
    now_playing: String,
    is_paused: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    BackwardPressed,
    ForwardPressed,
    SliderChanged(f32),
    InitiatePlay(String, u64),
    PlayAction,
    PauseAction,
    Tick,
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::Tick => {
                if self.is_paused {
                    return Command::none();
                }

                self.seconds_passed += 1;
                self.slider_value += 1.0;

                if self.seconds_passed >= self.total_duration {
                    self.slider_is_active = false;
                    self.slider_value = 0.0;
                    self.total_duration = 0;

                    self.formatted_current_duration = "0:00".to_string();
                    self.formatted_total_duration = "0:00".to_string();
                }

                self.formatted_current_duration = format::format_duration(self.seconds_passed);
                self.formatted_total_duration = format::format_duration(self.total_duration);

                Command::none()
            }
            Event::BackwardPressed => {
                println!("Backward pressed");
                Command::none()
            }
            Event::ForwardPressed => {
                println!("Forward pressed");
                Command::none()
            }

            Event::SliderChanged(value) => {
                self.slider_value = value;
                self.seconds_passed = value as u64;

                self.formatted_current_duration = format::format_duration(self.seconds_passed);
                self.formatted_total_duration = format::format_duration(self.total_duration);

                Command::none()
            }
            Event::InitiatePlay(text, total_duration) => {
                self.is_paused = false;
                self.slider_is_active = false; // ensure slider state is reset
                self.slider_value = 0.0;
                self.seconds_passed = 0;

                self.now_playing = text;
                self.slider_is_active = true;
                self.total_duration = total_duration;

                Command::none()
            }

            Event::PlayAction => {
                self.is_paused = false;
                Command::none()
            }

            Event::PauseAction => {
                self.is_paused = true;
                Command::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        let pause_or_play: Element<Event>;

        if self.is_paused {
            pause_or_play = assets::action(assets::play_icon(), "Play", Some(Event::PlayAction));
        } else {
            pause_or_play = assets::action(assets::pause_icon(), "Pause", Some(Event::PauseAction));
        }

        container(
            column![
                text(&self.now_playing),
                row![
                    assets::action(
                        assets::backward_icon(),
                        "Back",
                        Some(Event::BackwardPressed)
                    ),
                    pause_or_play,
                    assets::action(
                        assets::forward_icon(),
                        "Forward",
                        Some(Event::ForwardPressed)
                    ),
                ]
                .spacing(10),
                row![
                    text(&self.formatted_current_duration).size(14),
                    slider(
                        0.0..=self.total_duration as f32,
                        self.slider_value,
                        Event::SliderChanged
                    )
                    .step(1.0)
                    .width(400),
                    text(&self.formatted_total_duration).size(14),
                ]
                .align_items(Alignment::Center)
                .spacing(10),
            ]
            .align_items(Alignment::Center)
            .spacing(5),
        )
        .style(style::dynamic_colour)
        .width(Length::Fill)
        .height(100)
        .center_x()
        .padding(10)
        .into()
    }

    pub fn subscription(&self) -> iced::Subscription<Event> {
        if self.slider_is_active {
            return time::every(Duration::from_secs(1)).map(|_| Event::Tick);
        }

        iced::Subscription::none()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            formatted_current_duration: String::from("0:00"),
            formatted_total_duration: String::from("0:00"),
            slider_value: 0.0,
            total_duration: 0,
            seconds_passed: 1,
            slider_is_active: false,
            now_playing: String::from("Nothing is playing."),
            is_paused: false,
        }
    }
}
