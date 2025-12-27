use crate::manager::{command, dispatch};
use crate::AppWindow;
use log::error;

pub fn double_click_play(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_double_click_play(move |music_info| {
        match ui_sender.send(command::Command::Play(music_info)) {
            Ok(_) => (),
            Err(e) => error!("Error sending play command: {:?}", e),
        }
    })
}
