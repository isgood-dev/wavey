use iced::Command;

mod edit;
mod track_list;
mod settings;
mod download;

pub struct Pages {
    pub current_page: Page,

    track_list: track_list::State,
    edit: edit::State,
    settings: settings::State,
    download: download::State,
}

#[derive(Default)]
pub enum Page {
    #[default]
    TrackList,
    Edit,
    Settings,
    Download
}

#[derive(Debug, PartialEq)]
pub enum Event {
    TrackListPressed(track_list::Event),
    EditPressed(edit::Event),
    SettingsPressed(settings::Event),
    DownloadPressed(download::Event),
}

impl Pages {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::DownloadPressed(x) => {
                match x {
                    download::Event::OpenDownload => self.current_page = Page::Download,
                    download::Event::OpenEdit => self.current_page = Page::Edit,
                    download::Event::OpenSettings => self.current_page = Page::Settings,
                    download::Event::OpenTrackList => self.current_page = Page::TrackList,
                }
                
                self.download.update(x).map(Event::DownloadPressed)
            }

            Event::EditPressed(x) => {
                match x {
                    edit::Event::OpenDownload => self.current_page = Page::Download,
                    edit::Event::OpenEdit => self.current_page = Page::Edit,
                    edit::Event::OpenSettings => self.current_page = Page::Settings,
                    edit::Event::OpenTrackList => self.current_page = Page::TrackList,
                }

                self.edit.update(x).map(Event::EditPressed)
            }

            Event::SettingsPressed(x) => {
                match x {
                    settings::Event::OpenDownload => self.current_page = Page::Download,
                    settings::Event::OpenEdit => self.current_page = Page::Edit,
                    settings::Event::OpenSettings => self.current_page = Page::Settings,
                    settings::Event::OpenTrackList => self.current_page = Page::TrackList,
                }

                self.settings.update(x).map(Event::SettingsPressed)
            }

            Event::TrackListPressed(x) => {
                match x {
                    track_list::Event::OpenDownload => self.current_page = Page::Download,
                    track_list::Event::OpenEdit => self.current_page = Page::Edit,
                    track_list::Event::OpenSettings => self.current_page = Page::Settings,
                    track_list::Event::OpenTrackList => self.current_page = Page::TrackList,
                }

                self.track_list.update(x).map(Event::TrackListPressed)
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        match &self.current_page {
            Page::TrackList => {
                self.track_list.view().map(Event::TrackListPressed)
            }

            Page::Download => {
                self.download.view().map(Event::DownloadPressed)
            }

            Page::Edit => {
                self.edit.view().map(Event::EditPressed)
            }

            Page::Settings => {
                self.settings.view().map(Event::SettingsPressed)
            }           
        }
    }
}

impl Default for Pages {
    fn default() -> Self {
        Self {
            current_page: Default::default(),

            track_list: Default::default(),
            download: Default::default(),
            edit: Default::default(),
            settings: Default::default(),
        }
    }
}