use crate::App;
use crate::{AudioMetadata, load};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

// 关联前端界面显示音乐播放列表
pub fn show_music_list(ui: &App) {
    let music_list: ModelRc<AudioMetadata> = ModelRc::new(VecModel::default());
    ui.set_music_list(music_list.clone());

    let ui_weak = ui.as_weak();

    // 后续引入类似 SQLite 的简单存储来记录上一次的选择, 就不用每次都选择音乐文件了

    // 界面导入音乐回调
    ui.on_find_music_files(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let metadata_list = load::loader::get_audio_metadata_list();
            if !metadata_list.is_empty() {
                let mut items: Vec<AudioMetadata> = Vec::new();
                for metadata_info in metadata_list {
                    // 创建音乐项
                    let item = AudioMetadata {
                        index: metadata_info.index as i32,
                        title: SharedString::from(
                            metadata_info.title.unwrap_or("未知歌曲".to_string()),
                        ),
                        artist: SharedString::from(
                            metadata_info.artist.unwrap_or("未知艺术家".to_string()),
                        ),
                        album: SharedString::from(
                            metadata_info.album.unwrap_or("未知专辑".to_string()),
                        ),
                        duration: metadata_info.duration as f32,
                        duration_desc: SharedString::from(metadata_info.duration_desc),
                        path: SharedString::from(metadata_info.path),
                    };
                    items.push(item);
                }
                // 更新UI列表
                let vec_model = music_list
                    .as_any()
                    .downcast_ref::<VecModel<AudioMetadata>>()
                    .unwrap();
                vec_model.set_vec(items);
            }
        }
    });
}
