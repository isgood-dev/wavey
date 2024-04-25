mod download;
mod ui;

use iced::{Element, Command};

pub fn main() -> iced::Result {
    iced::program("Music Player", MusicPlayer::update, MusicPlayer::view)
        .window_size((900.0, 700.0))
        .run()
}

struct MusicPlayer {
    home_page: ui::track_list::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    HomePage(ui::track_list::Event)
}

impl MusicPlayer {
    fn new() -> Self {
        Self {
            home_page: Default::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
        //     Message::TestDownload => {
        //         println!("Download started");

        //         self.home_page.view().map(Message::HomePage);

        //         Command::none()

        //         // Command::perform(download_video("https://www.youtube.com/watch?v=wyDZc50mafw"), Message::DownloadRequested)
        //     }

        //     Message::DownloadRequested(result) => {
        //         println!("Video downloaded");

        //         Command::none()
        //     }

            Message::HomePage(x) => self.home_page.update(x).map(Message::HomePage),
        }
    }

    pub fn view(&self) -> Element<Message> {
        // let header = container(
        //     row![
        //         text("Music Player"),
        //         horizontal_space(),
        //         "Test",
        //     ]
        //     .padding(10)
        //     .align_items(Alignment::Center),
        // )
        // .style(|theme: &Theme| {
        //     let pallette = theme.extended_palette();

        //     container::Style::default()
        //         .with_border(pallette.background.strong.color, 1)
        // });

        // let sidebar = container(
        //     column!["Sidebar", button("Test Button 1"), button("Test Button 2")]
        //         .spacing(10)
        //         .padding(10)
        //         .width(200)
        //         .align_items(Alignment::Center),
        // )
        // .style(container::rounded_box)
        // .height(Length::Fill);

        // let content = container(
        //     scrollable(
        //         column![
        //             "Downloaded Songs",
        //             row![button("Test Button").on_press(Message::TestDownload), text("Test")].align_items(Alignment::Center).spacing(20)
        //         ]
        //         .spacing(40)
        //         .align_items(Alignment::Start)
        //         .width(Length::Fill),
        //     )
        //     .height(Length::Fill)
        // )
        // .padding(10);


        // column![header, row![sidebar, content]].into()        
        self.home_page.view().map(Message::HomePage)
    }
}

impl Default for MusicPlayer {
    fn default() -> Self {
        Self::new()
    }
}
