use iced::{widget::{row, column}, Command};
use rodio::{OutputStream, Sink};

use std::{fs::File, sync::mpsc};
use std::thread;

mod edit;
mod track_list;
mod settings;
mod download;
mod components;


pub struct Pages {
    pub current_page: Page,

    sidebar: components::sidebar::State,
    controls: components::control_bar::State,

    track_list: track_list::State,
    edit: edit::State,
    settings: settings::State,
    download: download::State,

    slider_value: f32,
    audio_playback_sender: mpsc::Sender<AudioEvent>,
    duration: String,    
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
pub enum UiEvent {
    SidebarPressed(components::sidebar::Event),
    ControlsPressed(components::control_bar::Event),

    TrackListPressed(track_list::Event),
    EditPressed(edit::Event),
    SettingsPressed(settings::Event),
    DownloadPressed(download::Event),

    PlaySuccess,    
}

#[derive(Debug, Clone)]
enum AudioEvent {
    Play(String),
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

            slider_value: 0.0,
            duration: String::from("0:00"),

            audio_playback_sender: sender,

        }
    }
    pub fn update(&mut self, message: UiEvent) -> Command<UiEvent> {
        match message {
            UiEvent::PlaySuccess => {
                println!("Play Successful");

                Command::none()
            }


            UiEvent::DownloadPressed(x) => self.download.update(x).map(UiEvent::DownloadPressed),
            UiEvent::EditPressed(x) => self.edit.update(x).map(UiEvent::EditPressed),
            UiEvent::SettingsPressed(x) => self.settings.update(x).map(UiEvent::SettingsPressed),
            UiEvent::TrackListPressed(ref x) => {
                println!("1");
                match x {
                    track_list::Event::PlayTrack(video_id) => {
                        println!("{}", video_id);
                        // return Command::perform(playback::play(self.audio.sink.clone(), video_id.to_string()), |_| Event::PlaySuccess);
                        self.audio_playback_sender
                            .send(AudioEvent::Play(video_id.to_string()))
                            .expect("Failed to send play command");
                    }
                    track_list::Event::Ignore => (),
                    
                }

                self.track_list.update(x.clone()).map(UiEvent::TrackListPressed)
            }
                      
            UiEvent::SidebarPressed(x) => {
                match x {
                    components::sidebar::Event::OpenDownload => self.current_page = Page::Download,
                    components::sidebar::Event::OpenEdit => self.current_page = Page::Edit,
                    components::sidebar::Event::OpenSettings => self.current_page = Page::Settings,
                    components::sidebar::Event::OpenTrackList => self.current_page = Page::TrackList,
                }

                self.sidebar.update(x).map(UiEvent::SidebarPressed)
            }
            
            UiEvent::ControlsPressed(x) => {
                self.controls.update(x).map(UiEvent::ControlsPressed)
            }
        }
    }

    pub fn view(&self) -> iced::Element<UiEvent> {
        match &self.current_page {
            Page::TrackList => {
                column![
                    row![
                        self.sidebar.view().map(UiEvent::SidebarPressed),
                        self.track_list.view().map(UiEvent::TrackListPressed),
                    ],
                    self.controls.view().map(UiEvent::ControlsPressed),
                ].into()
            }

            Page::Download => {
                column![
                    row![
                        self.sidebar.view().map(UiEvent::SidebarPressed),
                        self.download.view().map(UiEvent::DownloadPressed),
                    ],
                    self.controls.view().map(UiEvent::ControlsPressed),
                ].into()
            }

            Page::Edit => {
                column![
                    row![
                        self.sidebar.view().map(UiEvent::SidebarPressed),
                        self.edit.view().map(UiEvent::EditPressed),
                    ],
                    self.controls.view().map(UiEvent::ControlsPressed),
                ].into()
            }

            Page::Settings => {
                column![
                    row![
                        self.sidebar.view().map(UiEvent::SidebarPressed),
                        self.settings.view().map(UiEvent::SettingsPressed),
                    ],
                    self.controls.view().map(UiEvent::ControlsPressed),
                ].into()
            }           
        }
    }
}

fn process_audio_command(command: AudioEvent, sink: &Sink) {
    match command {
        AudioEvent::Play(video_id) => {
            println!("{}", format!("./assets/audio/{}.mp3", video_id.clone()));

            let file = File::open(format!("./assets/audio/{}.mp3", video_id)).unwrap();
            
            sink.append(rodio::Decoder::new(file).unwrap());
            sink.set_volume(1.0);
        }   

        AudioEvent::Pause => {
            sink.stop();
        }
    }
}

impl Default for Pages {
    fn default() -> Self {
        Self::new()
    }
}