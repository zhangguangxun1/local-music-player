use crate::audio::{playback, ui};
use crate::App;
use slint::{ComponentHandle, Model};
use std::sync::{Arc, Mutex};

// 按下播放
pub fn pressed_play(ui: &App, player: &Arc<Mutex<playback::AudioPlayer>>) {
    let ui_weak = ui.as_weak();
    let player_clone = player.clone();
    ui.on_pressed_play(move |_idx| {
        if let Some(ui) = ui_weak.upgrade() {
            let is_playing = ui.get_is_playing();
            if is_playing {
                return;
            }

            let player = player_clone.lock().unwrap();
            ui.set_is_playing(true);
            player.play();
        }
    })
}

// 按下暂停
pub fn pressed_pause(ui: &App, player: &Arc<Mutex<playback::AudioPlayer>>) {
    let ui_weak = ui.as_weak();
    let player_clone = player.clone();
    ui.on_pressed_pause(move |_idx| {
        if let Some(ui) = ui_weak.upgrade() {
            let is_playing = ui.get_is_playing();
            if !is_playing {
                return;
            }

            let player = player_clone.lock().unwrap();
            ui.set_is_playing(false);
            player.pause();
        }
    })
}

// 播放下一首
pub fn pressed_next(ui: &App, player: &Arc<Mutex<playback::AudioPlayer>>) {
    let ui_weak = ui.as_weak();
    let player_clone = player.clone();
    ui.on_pressed_next(move |idx| {
        if let Some(ui) = ui_weak.upgrade() {
            if let Some(music_info) = ui.get_music_list().row_data((idx + 1) as usize) {
                ui::play(&ui, &player_clone, music_info);
            }
        }
    })
}

// 播放上一首
pub fn pressed_prev(ui: &App, player: &Arc<Mutex<playback::AudioPlayer>>) {
    let ui_weak = ui.as_weak();
    let player_clone = player.clone();
    ui.on_pressed_prev(move |idx| {
        if let Some(ui) = ui_weak.upgrade() {
            if let Some(music_info) = ui.get_music_list().row_data((idx - 1) as usize) {
                ui::play(&ui, &player_clone, music_info);
            }
        }
    })
}

// 列表重复播放
pub fn pressed_repeat_list(ui: &App) {
    let ui_weak = ui.as_weak();
    ui.on_pressed_repeat_list(move |_idx| {
        if let Some(ui) = ui_weak.upgrade() {
            let repeat_list = ui.get_repeat_list();
            ui.set_repeat_list(!repeat_list);
        }
    })
}

// 列表乱序播放
pub fn pressed_shuffle_list(ui: &App) {
    let ui_weak = ui.as_weak();
    ui.on_pressed_shuffle_list(move |_idx| {
        if let Some(ui) = ui_weak.upgrade() {
            let shuffle_list = ui.get_shuffle_list();
            ui.set_shuffle_list(!shuffle_list);
        }
    })
}
