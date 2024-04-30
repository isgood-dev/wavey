use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text},
    Alignment, Command, Length, Theme,
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
        let header = container(
            row![text("Music Player"), horizontal_space(), "Test",]
                .padding(10)
                .align_items(Alignment::Center),
        )
        .style(|theme: &Theme| {
            let pallette = theme.extended_palette();

            container::Style::default().with_border(pallette.background.strong.color, 1)
        });

        let sidebar = container(
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
        .height(Length::Fill);


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

        column![header, row![sidebar, content]].into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}