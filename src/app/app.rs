use crate::audio::player;
use crate::manager::dispatch;
use crate::AppWindow;
use crate::{audio, load};
use log::error;
use slint::ComponentHandle;
use std::sync::{Arc, Mutex};

pub struct App {
    ui: Option<AppWindow>,
    dispatch: Option<dispatch::Dispatch>,
}

impl App {
    // 初始化前端
    fn new() -> Self {
        match AppWindow::new() {
            Ok(app) => Self {
                ui: Some(app),
                dispatch: None,
            },
            Err(e) => {
                error!("Created app err: {}", e);
                Self {
                    ui: None,
                    dispatch: None,
                }
            }
        }
    }

    // 加载全局只需初始化一次的资源
    fn load(&mut self) {
        if let Some(_ui) = &self.ui {
            if self.dispatch.is_none() {
                let (d, receiver) = dispatch::Dispatch::new();
                self.dispatch = Some(d);

                // 初始化音频等设备驱动程序, 首次播放时实际执行初始化
                let player = Arc::new(Mutex::new(player::Player::new()));

                dispatch::listen(&_ui, &player, receiver);
            }
        }
    }

    // 添加关联ui的请求和回调
    fn callback(&self) {
        if let Some(ui) = &self.ui {
            if let Some(d) = &self.dispatch {
                load::ui::select_files(&ui, &d);
                audio::ui::play(&ui, &d);
                audio::ui::current_play(&ui, &d);
                audio::ui::current_pause(&ui, &d);
                audio::ui::change_progress(&ui, &d);
                audio::ui::play_prev(&ui, &d);
                audio::ui::play_next(&ui, &d);
                audio::ui::repeat_list(ui, &d);
                audio::ui::shuffle_list(ui, &d);
            }
        }
    }

    // 运行前端应用
    fn run(&mut self) {
        if let Some(app) = &self.ui {
            match app.run() {
                Ok(_) => {}
                Err(e) => error!("Start run app err: {}", e),
            }
        } else {
            error!("No app found");
        }
    }
}

// 启动前端应用
pub fn start() {
    let mut app = App::new();
    app.load();
    app.callback();
    app.run();
}
