use iced::{Command, Element};

mod pages;

pub fn main() -> iced::Result {
    iced::program("Music Player", MusicPlayer::update, MusicPlayer::view)
        .run()
}

#[derive(Debug)]
enum Message {
    Pages(pages::Event),
}

struct MusicPlayer {
    pages: pages::Pages,
}

impl MusicPlayer {
    fn new() -> Self {
        Self {
            pages: Default::default(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Pages(x) => self.pages.update(x).map(Message::Pages),
        }
    }

    fn view(&self) -> Element<Message> {
        self.pages.view().map(Message::Pages).into()
    }
 }

 impl Default for MusicPlayer {
    fn default() -> Self {
        Self::new()
    }
 }