use core::db;
use core::json;

use iced::{window, Task, Element, Font, Settings, Subscription};

use image::GenericImageView;
use log::info;
use log4rs;

mod core;
mod ui;

pub fn main() -> iced::Result {
    // Setting the app icon.
    static ICON: &[u8] = include_bytes!("../assets/main.ico");

    let image = image::load_from_memory(ICON).unwrap();
    let (width, height) = image.dimensions();
    let rgba = image.into_rgba8();
    let icon = window::icon::from_rgba(rgba.into_raw(), width, height).unwrap();

    let settings = Settings {
        default_font: Font::with_name("Nunito"),
        window: iced::window::Settings {
            icon: Some(icon),
            ..Default::default()
        },
        ..Default::default()
    };

    iced::program("wavey", Wavey::update, Wavey::view)
        .settings(settings)
        .subscription(Wavey::subscription)
        .window_size((860.0, 680.0))
        .font(include_bytes!("../assets/icons.ttf").as_slice())
        .font(include_bytes!("../assets/font.ttf").as_slice())
        .theme(Wavey::theme)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Pages(ui::UiEvent),
}

struct Wavey {
    pages: ui::Pages,
}

impl Wavey {
    fn new() -> Self {
        log4rs::init_file("logging_config.yaml", Default::default()).unwrap();

        info!("Starting Wavey.");
        // Creates the database if it doesn't exist.
        if !db::check_database_exists() {
            info!("Creating database because it does not exist.");
            let _ = db::create_database_tables();
        }

        // Verifies validity of the data in the database.
        let _ = db::verify_data_integrity();

        if !json::check_exists() {
            info!("Creating settings file because it does not exist.");
            let _ = json::create_file();
        }

        Self {
            pages: Default::default(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Pages(x) => self.pages.update(x).map(Message::Pages),
        }
    }

    fn view(&self) -> Element<Message> {
        self.pages.view().map(Message::Pages).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        self.pages.subscription().map(Message::Pages)
    }

    fn theme(&self) -> iced::Theme {
        self.pages.theme()
    }
}

impl Default for Wavey {
    fn default() -> Self {
        Self::new()
    }
}
