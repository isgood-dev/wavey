use super::{add_audio, settings, track_list};

use iced::Command;

pub struct Pages {
    pub current_page: Page,

    track_list_page: track_list::State,
    settings_page: settings::State,
    add_audio_page: add_audio::State,
}

#[derive(Default)]
pub enum Page {
    #[default]
    TrackList,
    AddAudio,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Event {
    TrackListPressed(track_list::Event),
    AddAudioPressed(add_audio::Event),
    SettingsPressed(settings::Event),
}

impl Pages {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::TrackListPressed(x) => self.track_list_page.update(x).map(Event::TrackListPressed),
            Event::SettingsPressed(x) => self.settings_page.update(x).map(Event::SettingsPressed),
            Event::AddAudioPressed(x) => self.add_audio_page.update(x).map(Event::AddAudioPressed),
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        match &self.current_page {
            Page::AddAudio => {
                self.add_audio_page.view().map(Event::AddAudioPressed)
            }

            Page::Settings => {
                self.settings_page.view().map(Event::SettingsPressed)
            }

            Page::TrackList => {
                self.track_list_page.view().map(Event::TrackListPressed)
            }
        }
    }
}

impl Default for Pages {
    fn default() -> Self {
        Self {
            current_page: Page::TrackList, 
            track_list_page: Default::default(),
            settings_page: Default::default(),
            add_audio_page: Default::default(),
        }
    }
}