use crate::audio::player::Player;
use crate::load::loader;
use crate::manager::command;
use crate::{AppWindow, Attribute, audio, lyric};
use log::error;
use slint::{ComponentHandle, SharedString};
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
    player: &Arc<Mutex<Player>>,
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
                        Err(e) => error!("{}", e),
                    }
                }
                command::Command::Play(music_info) => {
                    let mut _player = player_clone.lock().unwrap();
                    _player.load(&music_info.path);

                    let lyrics = lyric::line::get_lyric_info_list(&music_info.path);

                    _player.play();

                    let ui_weak_clone = ui_weak.clone();
                    match slint::invoke_from_event_loop(move || {
                        if let Some(_ui) = ui_weak_clone.upgrade() {
                            let attribute = _ui.global::<Attribute>();

                            let cover = audio::album::get_album_cover(&music_info.path);

                            attribute.set_current_music_info(music_info);
                            attribute.set_playing(true);
                            attribute.set_current_lyric_info_list(lyrics.as_slice().into());
                            attribute.set_current_cover_image(cover);
                        }
                    }) {
                        Ok(_) => {}
                        Err(e) => error!("{}", e),
                    }
                }
            }
        }
    });
}
