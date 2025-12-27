use crate::audio::{album, player};
use crate::load::loader;
use crate::lyric::line;
use crate::manager::command;
use crate::{AppWindow, Attribute, PlayMode};
use log::error;
use slint::{ComponentHandle, Model, SharedString};
use std::sync::{mpsc};
use std::thread;

pub struct Dispatch {
    // 前端只需要往该通道发送需要操作的命令即可
    pub ui_sender: mpsc::Sender<command::Command>,
}

impl Dispatch {
    pub fn new() -> (Self, mpsc::Receiver<command::Command>) {
        let (ui_sender, back_receiver) = mpsc::channel();
        (Self { ui_sender }, back_receiver)
    }
}

// 使用一个固定线程监听用户的操作, 并对操作进行对应的渲染和信息获取
pub fn listen(
    ui: &AppWindow,
    receiver: mpsc::Receiver<command::Command>,
) {
    let ui_weak = ui.as_weak();

    thread::spawn(move || {
        // 初始化音频等设备驱动程序, 首次播放时实际执行初始化
        let mut player = player::Player::new();

        while let Ok(cmd) = receiver.recv() {
            match cmd {
                command::Command::SelectFiles => {
                    let (path, music_info_list) = loader::get_music_info_list();

                    let ui_weak_clone = ui_weak.clone();
                    match slint::invoke_from_event_loop(move || {
                        if let Some(_ui) = ui_weak_clone.upgrade() {
                            let attribute = _ui.global::<Attribute>();
                            attribute.set_music_folder(SharedString::from(path));
                            attribute.set_music_list(music_info_list.as_slice().into());
                        }
                    }) {
                        Ok(_) => {}
                        Err(e) => error!("{}", e),
                    }
                }
                command::Command::Play(mode, music_info) => {
                    player.load(&music_info.path);

                    let lyrics = line::get_lyric_info_list(&music_info.path);

                    player.play();

                    let ui_weak_clone = ui_weak.clone();
                    match slint::invoke_from_event_loop(move || {
                        if let Some(_ui) = ui_weak_clone.upgrade() {
                            let attribute = _ui.global::<Attribute>();

                            let cover = album::get_album_cover(&music_info.path);

                            attribute.set_current_music_info(music_info);
                            attribute.set_playing(true);
                            attribute.set_current_lyric_info_list(lyrics.as_slice().into());
                            attribute.set_current_cover_image(cover);

                            match mode {
                                PlayMode::Click => {}
                                PlayMode::Repeat => {
                                    _ui.invoke_play_prev();
                                }
                                PlayMode::Shuffle => _ui.invoke_play_next(),
                            }
                        }
                    }) {
                        Ok(_) => {}
                        Err(e) => error!("{}", e),
                    }
                }
                command::Command::PlayCurrent => {
                    if player.is_playing() {
                        return;
                    }
                    player.play();

                    let ui_weak_clone = ui_weak.clone();
                    match slint::invoke_from_event_loop(move || {
                        if let Some(_ui) = ui_weak_clone.upgrade() {
                            let attribute = _ui.global::<Attribute>();
                            attribute.set_playing(true);
                        }
                    }) {
                        Ok(_) => {}
                        Err(e) => error!("{}", e),
                    }
                }
                command::Command::Pause => {
                    if player.is_playing() {
                        player.pause();

                        let ui_weak_clone = ui_weak.clone();
                        match slint::invoke_from_event_loop(move || {
                            if let Some(_ui) = ui_weak_clone.upgrade() {
                                let attribute = _ui.global::<Attribute>();
                                attribute.set_playing(false);
                            }
                        }) {
                            Ok(_) => {}
                            Err(e) => error!("{}", e),
                        }
                    }
                }
                command::Command::ChangeProgress(val) => {
                    player.seek(val);

                    let ui_weak_clone = ui_weak.clone();
                    match slint::invoke_from_event_loop(move || {
                        if let Some(_ui) = ui_weak_clone.upgrade() {
                            let attribute = _ui.global::<Attribute>();
                            attribute.set_progress(val);
                        }
                    }) {
                        Ok(_) => {}
                        Err(e) => error!("{}", e),
                    }
                }
                command::Command::Prev => {
                    let ui_weak_clone = ui_weak.clone();
                    match slint::invoke_from_event_loop(move || {
                        if let Some(_ui) = ui_weak_clone.upgrade() {
                            let attribute = _ui.global::<Attribute>();
                            let music_info_list = attribute.get_music_list();
                            let music_info = attribute.get_current_music_info();
                            let idx = music_info.id - 1;
                            if let Some(prev_music_info) = music_info_list.row_data(idx as usize) {
                                attribute.set_current_music_info(prev_music_info.clone());
                                attribute.set_playing(false);
                                attribute.set_progress(0.0);
                                _ui.invoke_play(PlayMode::Click, prev_music_info);
                            }
                        }
                    }) {
                        Ok(_) => {}
                        Err(e) => error!("{}", e),
                    }
                }
                command::Command::Next => {
                    let ui_weak_clone = ui_weak.clone();
                    match slint::invoke_from_event_loop(move || {
                        if let Some(_ui) = ui_weak_clone.upgrade() {
                            let attribute = _ui.global::<Attribute>();
                            let music_info_list = attribute.get_music_list();
                            let music_info = attribute.get_current_music_info();
                            let idx = music_info.id + 1;
                            if let Some(next_music_info) = music_info_list.row_data(idx as usize) {
                                attribute.set_current_music_info(next_music_info.clone());
                                attribute.set_playing(false);
                                attribute.set_progress(0.0);
                                _ui.invoke_play(PlayMode::Click, next_music_info);
                            }
                        }
                    }) {
                        Ok(_) => {}
                        Err(e) => error!("{}", e),
                    }
                }
                command::Command::Repeat => {}
                command::Command::Shuffle => {}
            }
        }
    });
}
