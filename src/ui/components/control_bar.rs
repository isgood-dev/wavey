use iced::{
    time,
    widget::{column, container, row, slider, text},
    Alignment, Color, Command, Length,
};

use tokio::time::Duration;

use crate::core::format::interpolate_seconds_to_slider;

use super::icons::{action, backward_icon, forward_icon, play_icon};

pub struct State {
    total_duration: u64,
    slider_value: f32,
    seconds_passed: u64,
    slider_is_active: bool,
    now_playing: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    BackwardPressed,
    ForwardPressed,
    SliderChanged(f32),
    InitiatePlay(String, u64),
    PlayAction,
    PauseSlider,
    Tick,
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::Tick => {
                let value = interpolate_seconds_to_slider(self.seconds_passed, self.total_duration);

                if value > 100.0 {
                    self.slider_value = 0.0;
                    self.slider_is_active = false;
                    self.seconds_passed = 0;
                    return Command::none();
                }

                self.slider_value = value;
                self.seconds_passed += 1;

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
            Event::PauseSlider => {
                self.slider_is_active = false;
                Command::none()
            }

            Event::SliderChanged(value) => {
                self.slider_value = value;
                Command::none()
            }
            Event::InitiatePlay(text, total_duration) => {
                self.now_playing = text;
                self.slider_is_active = true;
                self.total_duration = total_duration;

                Command::none()
            }

            Event::PlayAction => {
                println!("Play action");
                Command::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        container(
            column![
                text(&self.now_playing).size(14),
                row![
                    action(backward_icon(), "Back", Some(Event::BackwardPressed)),
                    action(play_icon(), "Play", Some(Event::PlayAction)),
                    action(forward_icon(), "Forward", Some(Event::ForwardPressed)),
                ]
                .spacing(10),
                slider(0.0..=100.0, self.slider_value, Event::SliderChanged)
                    .step(1.0)
                    .width(350),
            ]
            .align_items(Alignment::Center)
            .spacing(10),
        )
        .style(|_theme| {
            container::Style::default()
                .with_background(Color::from_rgba(0.102, 0.102, 0.102, 1.0))
                .with_border(Color::from_rgb(255.0, 0.0, 0.0), 0)
        })
        .width(Length::Fill)
        .height(100)
        .padding(10)
        .center_x()
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
            slider_value: 0.0,
            total_duration: 0,
            seconds_passed: 1,
            slider_is_active: false,
            now_playing: String::from("Nothing is playing."),
        }
    }
}
