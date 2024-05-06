use iced::{
    widget::{column, container, row, slider}, Alignment, Color, Command, Length
};

use super::icons::{action, backward_icon, forward_icon, play_icon};
use crate::core::playback;

pub struct State {
    slider_value: f32,
    audio: playback::Audio,
    duration: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    BackwardPressed,
    ForwardPressed,
    PlayPressed,
    SliderChanged(f32),
    AudioPlayed,
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::BackwardPressed => {
                println!("Backward pressed");
                Command::none()
            }
            Event::ForwardPressed => {
                println!("Forward pressed");
                Command::none()
            }
            Event::PlayPressed => {
                Command::perform(playback::play(self.audio.sink.clone()), |_| Event::AudioPlayed)
            }
            Event::SliderChanged(value) => {
                self.slider_value = value;
                println!("Slider changed: {}", value);
                Command::none()
            }
            Event::AudioPlayed => {
                println!("Audio played");
                Command::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        container(
            column![
                row![
                    action(backward_icon(), "Back", Some(Event::BackwardPressed)),
                    action(play_icon(), "Play", Some(Event::PlayPressed)),
                    action(forward_icon(), "Forward", Some(Event::ForwardPressed)),
                ]
                .spacing(10),
                slider(0.0..=100.0, self.slider_value, Event::SliderChanged).step(1.0).width(350),
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
        .height(80)
        .padding(10)
        .center_x()
        .into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            slider_value: 0.0,
            audio: Default::default(),
            duration: String::from("0:00"),
        }
    }
}
