use iced::{
    widget::{button, column, container},
    Alignment, Command, Length
};

pub struct State {
    
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    OpenTrackList,
    OpenSettings,
    OpenEdit,
    OpenDownload,
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::OpenTrackList => Command::none(),
            Event::OpenSettings => Command::none(),
            Event::OpenEdit => Command::none(),
            Event::OpenDownload => Command::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        container(
            column![
                button("Home Page").on_press(Event::OpenTrackList),
                button("Edit Audio").on_press(Event::OpenEdit),
                button("Download Audio").on_press(Event::OpenDownload),
                button("Settings").on_press(Event::OpenSettings),
            ]
            .spacing(20)
            .padding(10)
            .width(200)
            .align_items(Alignment::Center),
        )
        .style(container::rounded_box)
        .height(Length::Fill)
        .into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}