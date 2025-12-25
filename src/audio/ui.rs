use crate::audio::playback;
use crate::lyric::lyric;
use crate::{App, AudioMetadata, LyricLine};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// 双击选中播放音乐
pub fn double_click_playback(ui: &App, player: &Arc<Mutex<playback::AudioPlayer>>) {
    let ui_weak = ui.as_weak();

    let player_clone = player.clone();

    ui.on_playback_item(move |idx| {
        if let Some(ui) = ui_weak.upgrade() {
            // 歌曲列表隐藏
            ui.set_content_visible(false);
            // 歌曲详情展示
            ui.set_info_visible(true);

            if let Some(music_info) = ui.get_music_list().row_data(idx as usize) {
                play(&ui, &player_clone, music_info);
            }
        }
    })
}

// 实际控制播放音乐
pub fn play(ui: &App, player_clone: &Arc<Mutex<playback::AudioPlayer>>, music_info: AudioMetadata) {
    let mut player = player_clone.lock().unwrap();

    let lyric_lines: ModelRc<LyricLine> = ModelRc::new(VecModel::default());
    ui.set_lyric_lines(lyric_lines.clone());

    // 加载待播放的音乐文件信息, 歌词跟音乐同名, 后缀为 .lrc 且在相同目录
    player.play_file(&music_info.path);

    ui.set_is_playing(true);
    ui.set_playback_progress(0.0f32);

    let ui_lyrics = get_ui_lyrics(player.get_lyrics());
    // 更新UI列表
    let vec_model = lyric_lines
        .as_any()
        .downcast_ref::<VecModel<LyricLine>>()
        .unwrap();
    vec_model.set_vec(ui_lyrics.clone());
    ui.set_current_line_index(0);

    // audioMetadata
    ui.set_audio_file(music_info);

    // 播放
    player.play();

    // 开始记时
    let start_time = Instant::now();

    while player.is_playing() && ui_lyrics.len() > 0 {
        let elapsed = start_time.elapsed().as_secs_f32();

        if let Some(pos) = ui_lyrics.iter().position(|x| x.time <= elapsed) {
            ui.set_current_line_index(pos as i32);
        }

        // 稍微休眠，减少 CPU 占用
        thread::sleep(Duration::from_millis(50));
    }
}

// 转换歌词列表对应ui界面
fn get_ui_lyrics(lyric_lines: Vec<lyric::LyricLine>) -> Vec<LyricLine> {
    let mut lyrics: Vec<LyricLine> = vec![];
    if !lyric_lines.is_empty() {
        for lyric_line in lyric_lines {
            let info = LyricLine {
                time: lyric_line.time as f32,
                text: SharedString::from(lyric_line.text),
            };
            lyrics.push(info);
        }
    }

    lyrics
}
