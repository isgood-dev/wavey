use std::{fs::File, sync::Arc};

pub struct Audio {
    _stream: rodio::OutputStream,
    _handle: rodio::OutputStreamHandle,
    pub sink: Arc<rodio::Sink>,
}

impl Audio {
    fn default() -> Self {
        let (_stream, _handle) = rodio::OutputStream::try_default().unwrap();
        let sink = Arc::new(rodio::Sink::try_new(&_handle).unwrap());
        Self {
            _stream,
            _handle,
            sink,
        }
    }
}

pub async fn play(sink: Arc<rodio::Sink>) {
    tokio::task::spawn_blocking(move || {
        let file = File::open("./assets/audio/output.mp3").unwrap();

        sink.append(rodio::Decoder::new(file).unwrap());
        sink.set_volume(1.0);

        sink.sleep_until_end();
    })
    .await
    .unwrap();
 }

impl Default for Audio {
    fn default() -> Self {
        Self::default()
    }
}

