use crate::AppWindow;
use crate::manager::{command, dispatch};
use log::error;

// 选择歌曲
pub fn select_files(ui: &AppWindow, dis: &dispatch::Dispatch) {
    let ui_sender = dis.ui_sender.clone();
    ui.on_select_files(
        move || match ui_sender.send(command::Command::SelectFiles) {
            Ok(_) => (),
            Err(e) => error!("Error sending select files command: {:?}", e),
        },
    );
}
