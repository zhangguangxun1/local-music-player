use crate::manager::{command, dispatch};
use crate::AppWindow;
use log::error;

pub fn playback(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_playback(move |music_info| {
        match ui_sender.send(command::Command::Playback(music_info)) {
            Ok(_) => (),
            Err(e) => error!("Error sending play command: {:?}", e),
        }
    })
}

pub fn current_play(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_current_play(move || match ui_sender.send(command::Command::Play) {
        Ok(_) => (),
        Err(e) => error!("Error sending current play command: {:?}", e),
    })
}

pub fn current_pause(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_current_pause(move || match ui_sender.send(command::Command::Pause) {
        Ok(_) => (),
        Err(e) => error!("Error sending pause command: {:?}", e),
    })
}

pub fn change_progress(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_change_progress(
        move |val| match ui_sender.send(command::Command::ChangeProgress(val)) {
            Ok(_) => (),
            Err(e) => error!("Error sending pause command: {:?}", e),
        },
    )
}

pub fn play_prev(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_play_prev(move || match ui_sender.send(command::Command::Prev) {
        Ok(_) => (),
        Err(e) => error!("Error sending play prev command: {:?}", e),
    })
}

pub fn play_next(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_play_next(move || match ui_sender.send(command::Command::Next) {
        Ok(_) => (),
        Err(e) => error!("Error sending play next command: {:?}", e),
    })
}

pub fn hope_repeat_play(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_hope_repeat_play(
        move |mode| match ui_sender.send(command::Command::HopeRepeat(mode)) {
            Ok(_) => (),
            Err(e) => error!("Error sending hope repeat command: {:?}", e),
        },
    )
}

pub fn hope_shuffle_play(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_hope_shuffle_play(
        move |mode| match ui_sender.send(command::Command::HopeShuffle(mode)) {
            Ok(_) => (),
            Err(e) => error!("Error sending hope shuffle command: {:?}", e),
        },
    )
}
