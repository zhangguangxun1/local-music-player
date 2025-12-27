use log::error;
use rodio::{OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub struct Player {
    sink: Option<Sink>,            // 复用同一个sink
    _stream: Option<OutputStream>, // 需要保持stream存活
}

impl Player {
    pub(crate) fn new() -> Self {
        Self {
            sink: None,
            _stream: None,
        }
    }

    pub fn load(&mut self, file_path: &str) {
        // 停止当前播放
        self.stop();

        // 创建新的stream和sink（只有第一次或需要重置时）
        if self.sink.is_none() {
            let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
                .expect("Failed to open default stream");
            let sink = Sink::connect_new(&stream_handle.mixer());
            self._stream = Some(stream_handle);
            self.sink = Some(sink);
        }

        // 加载新音频到sink
        if let Some(sink) = &self.sink {
            sink.append(
                rodio::Decoder::new(BufReader::new(
                    File::open(file_path).expect("Failed to open music file"),
                ))
                .unwrap(),
            );

            // 如果是暂停状态，先暂停
            if !self.is_playing() {
                sink.pause();
            }
        }
    }

    pub fn play(&self) {
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }

    pub fn pause(&self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }

    pub fn seek(&self, duration: f32) {
        if duration <= 0.0 {
            return;
        }
        if let Some(sink) = &self.sink {
            match sink.try_seek(Duration::from_secs_f32(duration)) {
                Ok(_) => (),
                Err(e) => error!("Error try seek: {:?}", e),
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
    }

    pub fn is_playing(&self) -> bool {
        self.sink.as_ref().map_or(false, |s| !s.is_paused())
    }
}
