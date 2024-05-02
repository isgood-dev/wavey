use iced::{window, Command, Element, Settings};
use image::GenericImageView;

mod ui;
mod core;

use core::sql::{self, DatabaseError};



pub fn main() -> iced::Result {
    static ICON: &[u8] = include_bytes!("../assets/main.ico");

    let image = image::load_from_memory(ICON).unwrap();
    let (width, height) = image.dimensions();
    let rgba = image.into_rgba8();
    let icon = window::icon::from_rgba(rgba.into_raw(), width, height).unwrap();

    let settings = Settings {
        window: iced::window::Settings {
            icon: Some(icon),
            ..Default::default()
        },
        ..Default::default()
    };

    iced::program("Music Player", MusicPlayer::update, MusicPlayer::view)
        .settings(settings)
        .window_size((800.0, 600.0))
        .font(include_bytes!("../assets/icons.ttf").as_slice())
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Pages(ui::Event),
    CreatedDatabase(Result<bool, DatabaseError>),
}

struct MusicPlayer {
    pages: ui::Pages,
}

impl MusicPlayer {
    fn new() -> Self {
        let _ = Command::perform(sql::create_database_tables(), Message::CreatedDatabase);

        Self {
            pages: Default::default(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Pages(x) => self.pages.update(x).map(Message::Pages),
            Message::CreatedDatabase(_) => {
                println!("Database was made.");

                Command::none()
            }
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