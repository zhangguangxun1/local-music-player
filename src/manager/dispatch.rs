use crate::audio::{album, player};
use crate::load::loader;
use crate::lyric::line;
use crate::manager::command;
use crate::{AppWindow, Attribute};
use log::error;
use rand::{Rng, rng};
use slint::{ComponentHandle, Model, SharedString, Weak};
use std::sync::{Arc, Mutex, mpsc};
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
    player: &Arc<Mutex<player::Player>>,
    receiver: mpsc::Receiver<command::Command>,
) {
    let ui_weak = ui.as_weak();
    let player_clone = player.clone();

    thread::spawn(move || {
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
                        Err(e) => error!("Dispatch command selectFiles err: {}", e),
                    }
                }
                command::Command::Playback(music_info) => {
                    let mut _player = player_clone.lock().unwrap();
                    _player.load(&music_info.path);

                    let lyrics = line::get_lyric_info_list(&music_info.path);

                    _player.play();

                    let ui_weak_clone = ui_weak.clone();
                    match slint::invoke_from_event_loop(move || {
                        if let Some(_ui) = ui_weak_clone.upgrade() {
                            let attribute = _ui.global::<Attribute>();

                            let cover = album::get_album_cover(&music_info.path);

                            attribute.set_current_music_info(music_info);
                            attribute.set_playing(true);
                            attribute.set_current_lyric_info_list(lyrics.as_slice().into());
                            attribute.set_current_cover_image(cover);
                        }
                    }) {
                        Ok(_) => {}
                        Err(e) => error!("Dispatch command playback err: {}", e),
                    }
                }
                command::Command::Play => {
                    let _player = player_clone.lock().unwrap();
                    if _player.is_playing() {
                        return;
                    }
                    _player.play();

                    let ui_weak_clone = ui_weak.clone();
                    match slint::invoke_from_event_loop(move || {
                        if let Some(_ui) = ui_weak_clone.upgrade() {
                            let attribute = _ui.global::<Attribute>();
                            attribute.set_playing(true);
                        }
                    }) {
                        Ok(_) => {}
                        Err(e) => error!("Dispatch command play err: {}", e),
                    }
                }
                command::Command::Pause => {
                    let _player = player_clone.lock().unwrap();
                    if _player.is_playing() {
                        _player.pause();

                        let ui_weak_clone = ui_weak.clone();
                        match slint::invoke_from_event_loop(move || {
                            if let Some(_ui) = ui_weak_clone.upgrade() {
                                let attribute = _ui.global::<Attribute>();
                                attribute.set_playing(false);
                            }
                        }) {
                            Ok(_) => {}
                            Err(e) => error!("Dispatch command pause err: {}", e),
                        }
                    }
                }
                command::Command::ChangeProgress(val) => {
                    let _player = player_clone.lock().unwrap();
                    _player.seek(val);

                    let ui_weak_clone = ui_weak.clone();
                    match slint::invoke_from_event_loop(move || {
                        if let Some(_ui) = ui_weak_clone.upgrade() {
                            let attribute = _ui.global::<Attribute>();
                            attribute.set_progress(val);
                        }
                    }) {
                        Ok(_) => {}
                        Err(e) => error!("Dispatch command change progress err: {}", e),
                    }
                }
                command::Command::Prev => {
                    let ui_weak_clone = ui_weak.clone();
                    pick_music_info_playback(ui_weak_clone, PickMode::Prev);
                }
                command::Command::Next => {
                    let ui_weak_clone = ui_weak.clone();
                    pick_music_info_playback(ui_weak_clone, PickMode::Next);
                }
                command::Command::HopeRepeat(mode) => {
                    let ui_weak_clone = ui_weak.clone();
                    match slint::invoke_from_event_loop(move || {
                        if let Some(_ui) = ui_weak_clone.upgrade() {
                            let attribute = _ui.global::<Attribute>();
                            attribute.set_is_repeat_play(mode);
                        }
                    }) {
                        Ok(_) => {}
                        Err(e) => error!("Dispatch command hope repeat play err: {}", e),
                    }
                }
                command::Command::HopeShuffle(mode) => {
                    let ui_weak_clone = ui_weak.clone();
                    match slint::invoke_from_event_loop(move || {
                        if let Some(_ui) = ui_weak_clone.upgrade() {
                            let attribute = _ui.global::<Attribute>();
                            attribute.set_is_shuffle_play(mode);
                        }
                    }) {
                        Ok(_) => {}
                        Err(e) => error!("Dispatch command hope shuffle play err: {}", e),
                    }
                }
            }
        }

        enum PickMode {
            Prev,
            Next,
        }

        // 根据是否设置重复播放和乱序播放, 选择下一首要播放的歌
        fn pick_music_info_playback(ui_weak: Weak<AppWindow>, pick_mode: PickMode) {
            match slint::invoke_from_event_loop(move || {
                if let Some(_ui) = ui_weak.upgrade() {
                    let attribute = _ui.global::<Attribute>();
                    let music_info_list = attribute.get_music_list();

                    let len = music_info_list.iter().len() as i32;

                    let index;
                    match pick_mode {
                        PickMode::Prev => {
                            let music_info = attribute.get_current_music_info();
                            index = music_info.id - 1;
                        }
                        PickMode::Next => {
                            // 优先处理乱序播放, 如果同时设置了列表重复依然按乱序播放处理
                            if attribute.get_is_shuffle_play() {
                                index = rng().random_range(0..len);
                            } else {
                                let music_info = attribute.get_current_music_info();
                                index = music_info.id + 1;
                            }
                        }
                    }

                    if let Some(pick_music_info) = music_info_list.row_data(index as usize) {
                        attribute.set_current_music_info(pick_music_info.clone());
                        attribute.set_playing(false);
                        attribute.set_progress(0.0);
                        _ui.invoke_playback(pick_music_info);
                    } else {
                        error!("Dispatch no find music info");
                    }
                }
            }) {
                Ok(_) => {}
                Err(e) => error!("Dispatch play next err: {}", e),
            }
        }
    });
}
