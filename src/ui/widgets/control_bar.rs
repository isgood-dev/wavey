use iced::{
    widget::{button, container, row},
    Command, Length
};

use super::icons::play_icon;

pub struct State {
    
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        container(row![
            action(back), // TODO: download back/forward icons
            action(play_icon(), "Play"),
            button("Backward"),
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