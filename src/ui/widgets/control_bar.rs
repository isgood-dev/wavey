use iced::{
    widget::{button, container, row, shader::wgpu::naga::back},
    Command, Length
};

use super::icons::{action, backward_icon, forward_icon, play_icon};

pub struct State {
    
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    BackwardPressed,
    ForwardPressed,
    PlayPressed,
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
                println!("Play pressed");
                Command::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        container(row![
            action(backward_icon(), "Back", Some(Event::BackwardPressed)), // TODO: download back/forward icons
            action(play_icon(), "Play", Some(Event::BackwardPressed)),
            action(forward_icon(), "Forward", Some(Event::ForwardPressed)),
        ])
        .style(container::rounded_box)
        .width(Length::Fill)
        .height(80)
        .center_x()
        .into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}