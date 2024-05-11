use iced::{window, Command, Element, Font, Settings};
use image::GenericImageView;

mod core;
mod ui;

use core::sql::{self, verify_data_integrity};

pub fn main() -> iced::Result {
    // Setting the app icon.
    static ICON: &[u8] = include_bytes!("../assets/main.ico");

    let image = image::load_from_memory(ICON).unwrap();
    let (width, height) = image.dimensions();
    let rgba = image.into_rgba8();
    let icon = window::icon::from_rgba(rgba.into_raw(), width, height).unwrap();

    let settings = Settings {
        default_font: Font::MONOSPACE,
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
    Pages(ui::UiEvent),
}

struct MusicPlayer {
    pages: ui::Pages,
}

impl MusicPlayer {
    fn new() -> Self {
        // Creates the database if it doesn't exist.
        if !sql::check_database_exists() {
            let _ = sql::create_database_tables();
        }

        // Verifies validity of the data in the database.
        let _ = verify_data_integrity();

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
