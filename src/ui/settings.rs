use iced::{
    widget::{ column, container, scrollable},
    Alignment, Command, Length,
};

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
        let content = container(
            scrollable(
                column![
                    "Settings",
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
        Self {}
    }
}