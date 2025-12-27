use crate::AppWindow;
use crate::manager::{command, dispatch};
use log::error;

pub fn play(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_play(move |mode, music_info| {
        match ui_sender.send(command::Command::Play(mode, music_info)) {
            Ok(_) => (),
            Err(e) => error!("Error sending play command: {:?}", e),
        }
    })
}

pub fn current_play(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_current_play(
        move || match ui_sender.send(command::Command::PlayCurrent) {
            Ok(_) => (),
            Err(e) => error!("Error sending current play command: {:?}", e),
        },
    )
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
        Err(e) => error!("Error sending play prve command: {:?}", e),
    })
}

pub fn play_next(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_play_next(move || match ui_sender.send(command::Command::Next) {
        Ok(_) => (),
        Err(e) => error!("Error sending play next command: {:?}", e),
    })
}

pub fn repeat_list(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_repeat_play(move || match ui_sender.send(command::Command::Repeat) {
        Ok(_) => (),
        Err(e) => error!("Error sending repeat command: {:?}", e),
    })
}

pub fn shuffle_list(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_shuffle_play(move || match ui_sender.send(command::Command::Shuffle) {
        Ok(_) => (),
        Err(e) => error!("Error sending shuffle command: {:?}", e),
    })
}
