use iced::{
    widget::{column, row},
    Command,
};
use rodio::{OutputStream, Sink};

use std::thread;
use std::{fs::File, sync::mpsc};

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

    PlaySuccess,
}

#[derive(Debug, Clone)]
enum AudioEvent {
    Queue(String, bool),
    Pause,
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
        }
    }
    pub fn update(&mut self, message: UiEvent) -> Command<UiEvent> {
        match message {
            UiEvent::PlaySuccess => {
                println!("Play Successful");

                Command::none()
            }

            UiEvent::DownloadPressed(x) => {
                match x.clone() {
                    download::Event::DownloadQueryReceived(data) => {
                        self.current_page = Page::Results;

                        let _ = self.results.update(results::Event::PopulateResults(data));
                    }
                    _ => (),
                }
                self.download.update(x).map(UiEvent::DownloadPressed)
            },
            UiEvent::EditPressed(x) => self.edit.update(x).map(UiEvent::EditPressed),
            UiEvent::SettingsPressed(x) => self.settings.update(x).map(UiEvent::SettingsPressed),
            UiEvent::TrackListPressed(ref x) => {
                println!("1");
                match x {
                    track_list::Event::PlayTrack(video_id) => {
                        println!("{}", video_id);
                        self.audio_playback_sender
                            .send(AudioEvent::Queue(video_id.clone().to_string(), true))
                            .expect("Failed to send play command");

                        let _ =
                            self.controls
                                .update(components::control_bar::Event::UpdateNowPlaying(
                                    video_id.to_string(),
                                ));
                    }
                    track_list::Event::Ignore => (),
                }

                self.track_list
                    .update(x.clone())
                    .map(UiEvent::TrackListPressed)
            }

            UiEvent::SidebarPressed(x) => {
                match x {
                    components::sidebar::Event::OpenDownload => self.current_page = Page::Download,
                    components::sidebar::Event::OpenEdit => self.current_page = Page::Edit,
                    components::sidebar::Event::OpenSettings => self.current_page = Page::Settings,
                    components::sidebar::Event::OpenTrackList => {
                        self.current_page = Page::TrackList
                    }
                }

                self.sidebar.update(x).map(UiEvent::SidebarPressed)
            }

            UiEvent::ControlsPressed(x) => self.controls.update(x).map(UiEvent::ControlsPressed),
            UiEvent::Results(x) => self.results.update(x).map(UiEvent::Results),
        }
    }

    pub fn view(&self) -> iced::Element<UiEvent> {
        match &self.current_page {
            Page::Results => column![
                row![
                    self.sidebar.view().map(UiEvent::SidebarPressed),
                    self.results.view().map(UiEvent::Results),
                ],
                self.controls.view().map(UiEvent::ControlsPressed),
            ]
            .into(),

            Page::TrackList => column![
                row![
                    self.sidebar.view().map(UiEvent::SidebarPressed),
                    self.track_list.view().map(UiEvent::TrackListPressed),
                ],
                self.controls.view().map(UiEvent::ControlsPressed),
            ]
            .into(),

            Page::Download => column![
                row![
                    self.sidebar.view().map(UiEvent::SidebarPressed),
                    self.download.view().map(UiEvent::DownloadPressed),
                ],
                self.controls.view().map(UiEvent::ControlsPressed),
            ]
            .into(),

            Page::Edit => column![
                row![
                    self.sidebar.view().map(UiEvent::SidebarPressed),
                    self.edit.view().map(UiEvent::EditPressed),
                ],
                self.controls.view().map(UiEvent::ControlsPressed),
            ]
            .into(),

            Page::Settings => column![
                row![
                    self.sidebar.view().map(UiEvent::SidebarPressed),
                    self.settings.view().map(UiEvent::SettingsPressed),
                ],
                self.controls.view().map(UiEvent::ControlsPressed),
            ]
            .into(),
        }
    }
}

fn process_audio_command(command: AudioEvent, sink: &Sink) {
    match command {
        AudioEvent::Pause => {
            sink.pause();
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
