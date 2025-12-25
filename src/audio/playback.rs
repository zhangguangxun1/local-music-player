use crate::lyric::lyric;
use rodio::{OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::fs;

pub struct AudioPlayer {
    sink: Option<Sink>,            // 复用同一个sink
    _stream: Option<OutputStream>, // 需要保持stream存活
    current_file: String,
    lrc_file: Option<String>,
}

impl AudioPlayer {
    pub(crate) fn new() -> Self {
        Self {
            sink: None,
            _stream: None,
            current_file: String::new(),
            lrc_file: None,
        }
    }

    pub fn play_file(&mut self, file_path: &str) {
        // 如果正在播放同一个文件，不重新加载
        if self.current_file == file_path && self.is_playing() {
            return;
        }

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
            self.current_file = file_path.to_string();
            self.lrc_file = lyric::get_lrc_file(file_path);

            // 如果是暂停状态，先暂停
            if !self.is_playing() {
                sink.pause();
            }
        }
    }

    pub fn get_lyrics(&self) -> Vec<lyric::LyricLine> {
        let mut lyrics: Vec<lyric::LyricLine> = vec![];
        if let Some(file) = &self.lrc_file {
            // 解析歌词
            let lrc_content = fs::read_to_string(file).expect("Failed to read file");
            lyrics = lyric::get_lyrics(&lrc_content);
        }
        lyrics
    }

    pub fn play(&self) {
        if let Some(sink) = &self.sink {
            // let mut lyrics: Vec<LyricLine> = vec![];
            // if let Some(file) = &self.lrc_file {
            //     // 解析歌词
            //     let lrc_content = fs::read_to_string(file).expect("Failed to read file");
            //     lyrics = lyric::get_lyrics(&lrc_content);
            // }

            // 开始记时
            // let start_time = Instant::now();

            sink.play();

            // while !sink.empty() && lyrics.len() > 0 {
            //     let elapsed = start_time.elapsed();
            //
            //     if let Some(pos) = lyrics.iter().position(|x| x.time <= elapsed) {
            //         let line = lyrics.remove(pos);
            //         println!("{}", line.text);
            //     }
            //
            //     // 稍微休眠，减少 CPU 占用
            //     thread::sleep(Duration::from_millis(50));
            // }
        }
    }

    pub fn pause(&self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }

    fn stop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
            self.current_file.clear();
        }
    }

    pub fn is_playing(&self) -> bool {
        self.sink.as_ref().map_or(false, |s| !s.is_paused())
    }
}
