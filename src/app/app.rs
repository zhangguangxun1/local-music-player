use crate::audio;
use crate::load;
use crate::manager::dispatch;
use crate::{AppWindow, Attribute};
use log::error;
use slint::{ComponentHandle, Model, Timer, TimerMode};
use std::process;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// 添加关联ui的请求和回调
fn callback(ui: &AppWindow, d: &dispatch::Dispatch) {
    load::ui::select_files(&ui, &d);
    audio::ui::playback(&ui, &d);
    audio::ui::current_play(&ui, &d);
    audio::ui::current_pause(&ui, &d);
    audio::ui::change_progress(&ui, &d);
    audio::ui::play_prev(&ui, &d);
    audio::ui::play_next(&ui, &d);
    audio::ui::hope_repeat_play(ui, &d);
    audio::ui::hope_shuffle_play(ui, &d);
}

// 启动前端应用
pub fn start() {
    match AppWindow::new() {
        Ok(ui) => {
            // 初始化音频等设备驱动程序, 首次播放时实际执行初始化
            // 这部分初始化方式 Mac 上编译不通过, 之前了解是 rodio 库对平台的支持还存在问题, Mac 上编译本身也有问题, 暂时不处理
            let player = Arc::new(Mutex::new(audio::player::Player::new()));

            // 初始化前端发送事件通道和后端接收通道
            let (d, receiver) = dispatch::Dispatch::new();

            // 加载全局只需初始化一次的资源
            dispatch::listen(&ui, &player, receiver);

            // 添加所有ui回调
            callback(&ui, &d);

            // 计时器关联ui操作, 只能在主线程内执行
            let ui_weak = ui.as_weak();
            let timer = Timer::default();
            timer.start(TimerMode::Repeated, Duration::from_millis(800), move || {
                let ui_weak_clone = ui_weak.clone();
                let _player = player.lock().unwrap();

                // 如果没有播放也不为空, 说明是暂停状态不处理
                if !_player.is_empty() && !_player.is_playing() {
                    return;
                }

                if let Some(_ui) = ui_weak_clone.upgrade() {
                    let attribute = _ui.global::<Attribute>();

                    // 歌曲正在播放则处理歌词
                    if _player.is_playing() {
                        let pos = _player.get_pos();
                        attribute.set_progress(pos);

                        let lyrics = attribute.get_current_lyric_info_list();
                        let pos_millis: f32 = pos * 1000.0;
                        if lyrics.iter().len() > 0 {
                            // pos_millis 接近实际播放的时间
                            // x.time 是实际对应歌词设定的时间, 有的可能本身就有稍快或者稍慢
                            if let Some(mut _pos) = lyrics.iter().position(|x| x.time >= pos_millis)
                            {
                                // 本身是查找最接近当前时间的行, 故相当于快进了一步, 所以回退一步, 看起来往前遍历和往后遍历应该是差不多的, 后续在考虑优化
                                if _pos > 0 {
                                    _pos -= 1;
                                }
                                attribute.set_current_lyric_index(_pos as i32);
                            }
                        }
                    }

                    // 播放队列为空则检查是否需要播放后续歌曲
                    if _player.is_empty() {
                        if attribute.get_is_shuffle_play() || attribute.get_is_repeat_play() {
                            _ui.invoke_play_next();
                        }
                    }
                }
            });

            match ui.run() {
                Ok(_) => {}
                Err(e) => error!("Start run app err: {}", e),
            }
        }
        Err(e) => {
            error!("Create ui err and exit: {}", e);
            process::exit(1);
        }
    }
}
