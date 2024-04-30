use iced::{widget::row, Command};

mod edit;
mod track_list;
mod settings;
mod download;
mod sidebar;

pub struct Pages {
    pub current_page: Page,

    sidebar: sidebar::State,

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
    SidebarPressed(sidebar::Event),
}

impl Pages {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::DownloadPressed(x) => self.download.update(x).map(Event::DownloadPressed),
            Event::EditPressed(x) => self.edit.update(x).map(Event::EditPressed),
            Event::SettingsPressed(x) => self.settings.update(x).map(Event::SettingsPressed),
            Event::TrackListPressed(x) =>  self.track_list.update(x).map(Event::TrackListPressed),            
            Event::SidebarPressed(x) => {
                match x {
                    sidebar::Event::OpenDownload => self.current_page = Page::Download,
                    sidebar::Event::OpenEdit => self.current_page = Page::Edit,
                    sidebar::Event::OpenSettings => self.current_page = Page::Settings,
                    sidebar::Event::OpenTrackList => self.current_page = Page::TrackList,
                }

                self.sidebar.update(x).map(Event::SidebarPressed)
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        match &self.current_page {
            Page::TrackList => {
                row![
                    self.sidebar.view().map(Event::SidebarPressed),
                    self.track_list.view().map(Event::TrackListPressed),
                ].into()
            }

            Page::Download => {
                row![
                    self.sidebar.view().map(Event::SidebarPressed),
                    self.download.view().map(Event::DownloadPressed),
                ].into()
            }

            Page::Edit => {
                row![
                    self.sidebar.view().map(Event::SidebarPressed),
                    self.edit.view().map(Event::EditPressed),
                ].into()
            }

            Page::Settings => {
                row![
                    self.sidebar.view().map(Event::SidebarPressed),
                    self.settings.view().map(Event::SettingsPressed),
                ].into()
            }           
        }
    }
}

impl Default for Pages {
    fn default() -> Self {
        Self {
            current_page: Default::default(),

            sidebar: Default::default(),

            track_list: Default::default(),
            download: Default::default(),
            edit: Default::default(),
            settings: Default::default(),
            
        }
    }
}