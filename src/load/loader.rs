use crate::MusicInfo;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::prelude::Accessor;
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult};
use slint::SharedString;
use std::fs;
use std::path::PathBuf;

// 支持的音乐文件格式
const AUDIO_EXTENSIONS: &[&str] = &[
    "mp3", "wav", "flac", "ogg", "m4a", "aac", "wma", "opus", "aiff", "alac",
];

// 检查是否为音频文件
fn is_audio_file(path: &PathBuf) -> bool {
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return AUDIO_EXTENSIONS.contains(&ext_str.to_lowercase().as_str());
        }
    }
    false
}

// 扫描音频文件, 不递归扫描, 直接指定文件夹即可
fn scan_audio_files(dir: &PathBuf, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if is_audio_file(&path) {
                files.push(path);
            }
        }
    }
}

// 选择文件类型
enum SelectionType {
    Folder, // 文件夹
    Files,  // 文件
}

// 因为无法区分用户具体选择的文件还是文件夹, 故弹框提示选择
fn get_selection_type() -> Option<SelectionType> {
    let single_file = "选择文件".to_string();
    let folder = "选择文件夹".to_string();

    let result = MessageDialog::new()
        .set_title("选择文件类型")
        .set_description("指定你要添加的内容")
        .set_buttons(MessageButtons::OkCancelCustom(
            folder.clone(),
            single_file.clone(),
        ))
        .show();

    match result {
        MessageDialogResult::Yes => None,
        MessageDialogResult::No => None,
        MessageDialogResult::Ok => None,
        MessageDialogResult::Cancel => None,
        MessageDialogResult::Custom(custom) => {
            if custom == single_file {
                Some(SelectionType::Files)
            } else if custom == folder {
                Some(SelectionType::Folder)
            } else {
                None
            }
        }
    }
}

// 根据用户选择解析歌曲文件
fn get_audio_files() -> (String, Vec<PathBuf>) {
    let mut path: String = "".to_string();
    let mut audio_files = Vec::new();

    if let Some(selection_type) = get_selection_type() {
        match selection_type {
            SelectionType::Files => {
                if let Some(files) = FileDialog::new().set_title("选择文件").pick_files() {
                    for file in files {
                        if is_audio_file(&file) {
                            audio_files.push(file);
                        }
                    }
                }
            }
            SelectionType::Folder => {
                if let Some(folder) = FileDialog::new().set_title("选择文件夹").pick_folder() {
                    path = folder.to_str().unwrap_or("").to_string();
                    // 扫描文件夹中的音频文件
                    scan_audio_files(&folder, &mut audio_files);
                }
            }
        }
    }

    (path, audio_files)
}

// 音频元数据
pub struct AudioMetadata {
    pub path: String,  // 记录原始路径方便查找歌词文件, 同级别目录
    pub duration: f32, // 时长
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

// 解析音乐文件元数据
fn get_audio_metadata(path: &PathBuf) -> Option<AudioMetadata> {
    if let Ok(tagged_file) = lofty::read_from_path(path) {
        let properties = tagged_file.properties();
        let tag = tagged_file.primary_tag();

        let duration = properties.duration().as_secs_f32();
        Some(AudioMetadata {
            path: path.display().to_string(),
            duration,
            title: tag.and_then(|t| t.title().map(String::from)),
            artist: tag.and_then(|t| t.artist().map(String::from)),
            album: tag.and_then(|t| t.album().map(String::from)),
        })
    } else {
        None
    }
}

// 获取音乐文件元数据信息列表
fn get_audio_metadata_list() -> (String, Vec<AudioMetadata>) {
    let (path, audio_files) = get_audio_files();
    let mut audio_metadata_list: Vec<AudioMetadata> = Vec::new();
    for (_, audio_file) in audio_files.into_iter().enumerate() {
        if let Some(metadata) = get_audio_metadata(&audio_file) {
            audio_metadata_list.push(metadata);
        }
    }
    (path, audio_metadata_list)
}

// 获取歌曲列表
pub fn get_music_info_list() -> (String, Vec<MusicInfo>) {
    let (path, metadata_list) = get_audio_metadata_list();
    let mut music_info_list: Vec<MusicInfo> = Vec::new();
    for (idx, metadata) in metadata_list.into_iter().enumerate() {
        music_info_list.push(MusicInfo {
            id: idx as i32,
            album: SharedString::from(metadata.album.unwrap_or("".to_string())),
            artist: SharedString::from(metadata.artist.unwrap_or("".to_string())),
            duration: metadata.duration,
            path: SharedString::from(metadata.path),
            title: SharedString::from(metadata.title.unwrap_or("".to_string())),
        });
    }

    (path, music_info_list)
}

#[cfg(test)]
mod tests {
    use crate::load::loader::get_audio_files;

    #[test]
    fn test_get_audio_files() {
        let files = get_audio_files();
        println!("{:#?}", files);
    }
}
