// 在 Windows 发布版本中，除了 Slint 窗口外，还可以防止控制台窗口出现，
// 例如通过文件管理器启动应用程序时。
// 在其他平台上忽略此设置
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod audio;
mod load;
mod lyric;
mod manager;
mod util;

slint::include_modules!();

fn main() {
    // 初始化日志配置, 暂时只处理控制台输出, 并且仅有 error 及以上级别会输出
    env_logger::init();

    // 确保该名称下仅启动一个实例程序
    if !util::instance::check_is_single_instance("Local-music-player") {
        return;
    }

    // 启动应用
    app::app::start();
}
