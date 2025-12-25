// 在 Windows 发布版本中，除了 Slint 窗口外，还可以防止控制台窗口出现，
// 例如通过文件管理器启动应用程序时。
// 在其他平台上忽略此设置
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod info;
mod load;
mod lyric;

use crate::audio::playback;
use std::error::Error;
use std::sync::{Arc, Mutex};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    // 创建应用
    let ui = App::new()?;

    // 初始化音频等设备驱动程序, 首次播放时实际执行初始化
    let player = Arc::new(Mutex::new(playback::AudioPlayer::new()));

    // 选择音乐文件夹并显示歌曲列表
    load::ui::show_music_list(&ui);

    // 双击选中播放选中的音乐, 并跳转到详情
    audio::ui::double_click_playback(&ui, &player);

    // 详情页播放音乐
    info::ui::pressed_play(&ui, &player);
    info::ui::pressed_pause(&ui, &player);
    info::ui::pressed_next(&ui, &player);
    info::ui::pressed_prev(&ui, &player);
    info::ui::pressed_repeat_list(&ui);
    info::ui::pressed_shuffle_list(&ui);

    ui.run()?;

    Ok(())
}
