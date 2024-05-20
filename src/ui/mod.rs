use std::fs::File;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::core::youtube::YouTubeError;
use components::toast::{self, Status, Toast};
use components::theme::{match_theme, Themes};

use iced::widget::{column, row};
use iced::{Command, Subscription};

use rodio::{OutputStream, Sink};

mod components;
mod download;
mod edit;
mod results;
mod settings;
mod track_list;

pub struct Pages {
    pub current_page: Page,

    sidebar: components::sidebar::State,
    controls: components::control_bar::State,

    track_list: track_list::State,
    edit: edit::State,
    settings: settings::State,
    download: download::State,
    results: results::State,

    audio_playback_sender: mpsc::Sender<AudioEvent>,

    toasts: Vec<Toast>,
}

#[derive(Default)]
pub enum Page {
    #[default]
    TrackList,
    Edit,
    Settings,
    Download,
    Results,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UiEvent {
    SidebarPressed(components::sidebar::Event),
    ControlsPressed(components::control_bar::Event),

    TrackListPressed(track_list::Event),
    EditPressed(edit::Event),
    SettingsPressed(settings::Event),
    DownloadPressed(download::Event),
    Results(results::Event),

    CloseToast(usize),
}

#[derive(Debug, Clone)]
enum AudioEvent {
    Queue(String, bool),
    SeekTo(u64),
    Pause,
    Resume,
}

impl Pages {
    pub fn new() -> Self {
        let (sender, reciever) = mpsc::channel();

        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            loop {
                if let Ok(command) = reciever.try_recv() {
                    process_audio_command(command, &sink);
                }

                thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        Self {
            current_page: Default::default(),

            sidebar: Default::default(),
            controls: Default::default(),

            track_list: Default::default(),
            download: Default::default(),
            edit: Default::default(),
            settings: Default::default(),
            results: Default::default(),

            audio_playback_sender: sender,
            toasts: vec![],
        }
    }
    pub fn update(&mut self, message: UiEvent) -> Command<UiEvent> {
        match message {
            UiEvent::CloseToast(index) => {
                self.toasts.remove(index);

                Command::none()
            }

            UiEvent::DownloadPressed(event) => {
                let download_command = self
                    .download
                    .update(event.clone())
                    .map(UiEvent::DownloadPressed);
                match event {
                    download::Event::DownloadQueryReceived(data) => {
                        self.current_page = Page::Results;

                        let data = match data {
                            Ok(data) => data,
                            Err(error) => {
                                match error {
                                    YouTubeError::NetworkError => self.toasts.push(Toast {
                                        title: "Network Error".into(),
                                        body: "Failed to fetch search results".into(),
                                        status: Status::Danger,
                                    }),
                                    YouTubeError::UnknownError => self.toasts.push(Toast {
                                        title: "Unknown Error".into(),
                                        body: "An unknown error occurred".into(),
                                        status: Status::Danger,
                                    }),
                                    YouTubeError::VideoNotFound => self.toasts.push(Toast {
                                        title: "Video Not Found".into(),
                                        body: "The video you are looking for was not found".into(),
                                        status: Status::Danger,
                                    }),
                                }
                                return download_command;
                            }
                        };

                        Command::batch(vec![
                            self.results
                                .update(results::Event::PopulateResults(data))
                                .map(UiEvent::Results),
                            download_command,
                        ])
                    }
                    _ => download_command,
                }
            }
            UiEvent::EditPressed(event) => self.edit.update(event).map(UiEvent::EditPressed),
            UiEvent::SettingsPressed(event) => {
                match event {
                    settings::Event::ThemeSelected(theme) => {
                        self.theme(Some(theme));
                    }
                }
                self.settings.update(event).map(UiEvent::SettingsPressed)
            }
            UiEvent::TrackListPressed(ref event) => {
                let track_list_command = self
                    .track_list
                    .update(event.clone())
                    .map(UiEvent::TrackListPressed);
                match event {
                    track_list::Event::PlayTrack(video_id, display_name, duration) => {
                        self.audio_playback_sender
                            .send(AudioEvent::Queue(video_id.clone().to_string(), true))
                            .expect("Failed to send play command");

                        Command::batch(vec![
                            self.controls
                                .update(components::control_bar::Event::InitiatePlay(
                                    display_name.to_string(),
                                    *duration,
                                ))
                                .map(UiEvent::ControlsPressed),
                            track_list_command,
                        ])
                    }
                    _ => track_list_command,
                }
            }

            UiEvent::SidebarPressed(event) => {
                match event {
                    components::sidebar::Event::OpenDownload => self.current_page = Page::Download,
                    components::sidebar::Event::OpenEdit => self.current_page = Page::Edit,
                    components::sidebar::Event::OpenSettings => self.current_page = Page::Settings,
                    components::sidebar::Event::OpenTrackList => {
                        self.current_page = Page::TrackList
                    }
                }

                self.sidebar.update(event).map(UiEvent::SidebarPressed)
            }

            UiEvent::ControlsPressed(event) => {
                match event {
                    components::control_bar::Event::SliderChanged(value) => {
                        self.audio_playback_sender
                            .send(AudioEvent::SeekTo(value as u64))
                            .expect("Failed to send seek command");
                    }
                    components::control_bar::Event::PauseAction => {
                        self.audio_playback_sender
                            .send(AudioEvent::Pause)
                            .expect("Failed to send pause command");
                    }
                    components::control_bar::Event::PlayAction => {
                        self.audio_playback_sender
                            .send(AudioEvent::Resume)
                            .expect("Failed to send play command");
                    }
                    _ => (),
                }
                self.controls.update(event).map(UiEvent::ControlsPressed)
            }
            UiEvent::Results(event) => self.results.update(event).map(UiEvent::Results),
        }
    }

    pub fn view(&self) -> iced::Element<UiEvent> {
        match &self.current_page {
            Page::Results => {
                let content = column![
                    row![
                        self.sidebar.view().map(UiEvent::SidebarPressed),
                        self.results.view().map(UiEvent::Results),
                    ],
                    self.controls.view().map(UiEvent::ControlsPressed),
                ];

                toast::Manager::new(content, &self.toasts, UiEvent::CloseToast).into()
            }

            Page::TrackList => {
                let content = column![
                    row![
                        self.sidebar.view().map(UiEvent::SidebarPressed),
                        self.track_list.view().map(UiEvent::TrackListPressed),
                    ],
                    self.controls.view().map(UiEvent::ControlsPressed),
                ];

                toast::Manager::new(content, &self.toasts, UiEvent::CloseToast).into()
            }

            Page::Download => {
                let content = column![
                    row![
                        self.sidebar.view().map(UiEvent::SidebarPressed),
                        self.download.view().map(UiEvent::DownloadPressed),
                    ],
                    self.controls.view().map(UiEvent::ControlsPressed),
                ];

                toast::Manager::new(content, &self.toasts, UiEvent::CloseToast).into()
            }

            Page::Edit => {
                let content = column![
                    row![
                        self.sidebar.view().map(UiEvent::SidebarPressed),
                        self.edit.view().map(UiEvent::EditPressed),
                    ],
                    self.controls.view().map(UiEvent::ControlsPressed),
                ];

                toast::Manager::new(content, &self.toasts, UiEvent::CloseToast).into()
            }

            Page::Settings => {
                let content = column![
                    row![
                        self.sidebar.view().map(UiEvent::SidebarPressed),
                        self.settings.view().map(UiEvent::SettingsPressed),
                    ],
                    self.controls.view().map(UiEvent::ControlsPressed),
                ];

                toast::Manager::new(content, &self.toasts, UiEvent::CloseToast).into()
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<UiEvent> {
        Subscription::batch(vec![
            self.track_list
                .subscription()
                .map(UiEvent::TrackListPressed),
            self.controls.subscription().map(UiEvent::ControlsPressed),
        ])
    }

    pub fn theme(&self, new: Option<Themes>) -> iced::Theme {
        match_theme(new)
    }
}

fn process_audio_command(command: AudioEvent, sink: &Sink) {
    match command {
        AudioEvent::SeekTo(position) => {
            let try_seek = sink.try_seek(Duration::from_secs(position));

            match try_seek {
                Ok(_) => (),
                Err(_) => {
                    println!("Failed to seek")
                }
            }
        }
        AudioEvent::Pause => {
            sink.pause();
        }

        AudioEvent::Resume => {
            sink.play();
        }

        AudioEvent::Queue(video_id, force) => {
            if force {
                sink.stop();
            }

            let file = File::open(format!("./assets/audio/{}.mp3", video_id)).unwrap();

            sink.append(rodio::Decoder::new(file).unwrap());

            if force {
                sink.play();
            }
        }
    }
}

impl Default for Pages {
    fn default() -> Self {
        Self::new()
    }
}
