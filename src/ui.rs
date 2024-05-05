use iced::{widget::{row, column}, Command};

mod edit;
mod track_list;
mod settings;
mod download;
mod widgets;


pub struct Pages {
    pub current_page: Page,

    sidebar: widgets::sidebar::State,
    controls: widgets::control_bar::State,

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

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    SidebarPressed(widgets::sidebar::Event),
    ControlsPressed(widgets::control_bar::Event),

    TrackListPressed(track_list::Event),
    EditPressed(edit::Event),
    SettingsPressed(settings::Event),
    DownloadPressed(download::Event),
    
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
                    widgets::sidebar::Event::OpenDownload => self.current_page = Page::Download,
                    widgets::sidebar::Event::OpenEdit => self.current_page = Page::Edit,
                    widgets::sidebar::Event::OpenSettings => self.current_page = Page::Settings,
                    widgets::sidebar::Event::OpenTrackList => self.current_page = Page::TrackList,
                }

                self.sidebar.update(x).map(Event::SidebarPressed)
            }
            
            Event::ControlsPressed(x) => {
                self.controls.update(x).map(Event::ControlsPressed)
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        match &self.current_page {
            Page::TrackList => {
                column![
                    row![
                        self.sidebar.view().map(Event::SidebarPressed),
                        self.track_list.view().map(Event::TrackListPressed),
                    ],
                    self.controls.view().map(Event::ControlsPressed),
                ].into()
            }

            Page::Download => {
                column![
                    row![
                        self.sidebar.view().map(Event::SidebarPressed),
                        self.download.view().map(Event::DownloadPressed),
                    ],
                    self.controls.view().map(Event::ControlsPressed),
                ].into()
            }

            Page::Edit => {
                column![
                    row![
                        self.sidebar.view().map(Event::SidebarPressed),
                        self.edit.view().map(Event::EditPressed),
                    ],
                    self.controls.view().map(Event::ControlsPressed),
                ].into()
            }

            Page::Settings => {
                column![
                    row![
                        self.sidebar.view().map(Event::SidebarPressed),
                        self.settings.view().map(Event::SettingsPressed),
                    ],
                    self.controls.view().map(Event::ControlsPressed),
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
            controls: Default::default(),

            track_list: Default::default(),
            download: Default::default(),
            edit: Default::default(),
            settings: Default::default(),

        }
    }
}