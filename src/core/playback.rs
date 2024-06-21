use std::fs::File;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

use rodio::{OutputStream, Sink};

#[derive(Debug, Clone)]
pub enum AudioEvent {
    Queue(String, bool),
    SeekTo(u64),
    SetVolume(f32),
    PauseToggle,
    Mute,
    Unmute,
}

pub fn start_receiver(reciever: mpsc::Receiver<AudioEvent>) {
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
}

fn process_audio_command(command: AudioEvent, sink: &Sink) {
    match command {
        AudioEvent::SetVolume(volume) => {
            sink.set_volume(volume);
        }

        AudioEvent::Mute => {
            sink.set_volume(0.0);
        }

        AudioEvent::Unmute => {
            sink.set_volume(0.5);
        }

        AudioEvent::SeekTo(position) => {
            let try_seek = sink.try_seek(Duration::from_secs(position));

            match try_seek {
                Ok(_) => (),
                Err(_) => {
                    println!("Failed to seek")
                }
            }
        }
        AudioEvent::PauseToggle => {
            if sink.is_paused() {
                sink.play();
            } else {
                sink.pause();
            }
        }

        AudioEvent::Queue(video_id, force) => {
            if force {
                sink.stop();
            }

            let file = File::open(format!("./data/audio/{}.mp3", video_id)).unwrap();

            sink.append(rodio::Decoder::new(file).unwrap());

            if force {
                sink.play();
            }
        }
    }
}