use iced::{
    widget::{column, container, scrollable, text, image, Container},
    Alignment, Command, Length, 
};
pub struct State {
    
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Ignore,
}

impl State {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::Ignore => Command::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        let content = container(
            scrollable(
                column![
                    text("Select")
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
        Self::new()
    }
}

fn thumbnail<'a>(url: &'a str) -> Container<'a, Event> {
    container(
        image(url)
    ).center_x()
}