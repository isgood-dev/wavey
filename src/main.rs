use iced::{Command, Element};

mod pages;

pub fn main() -> iced::Result {
    iced::program("Music Player", MusicPlayer::update, MusicPlayer::view)
        .run()
}

#[derive(Debug, Clone)]
enum Message {}

struct MusicPlayer {}

impl MusicPlayer {
    fn new() -> Self {
        Self {
            // ???
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            // ???
        }
    }

    fn view(&self) -> Element<Message> {
        // ???
    }
 }

 impl Default for MusicPlayer {
    fn default() -> Self {
        Self::new()
    }
 }