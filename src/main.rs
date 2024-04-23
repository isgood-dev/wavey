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
        self.home_page.view().map(Message::HomePage)
    }
}

impl Default for MusicPlayer {
    fn default() -> Self {
        Self::new()
    }
}
