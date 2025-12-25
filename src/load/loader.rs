use lofty;
use lofty::prelude::{Accessor, AudioFile, TaggedFileExt};
use rfd::FileDialog;
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

// 音频元数据
pub struct AudioMetadata {
    pub index: u32,
    pub path: String,  // 记录原始路径方便查找歌词文件, 同级别目录
    pub duration: f64, // 时长
    pub duration_desc: String,
    pub sample_rate: u32, // 采样率
    pub channels: u8,     // 频道
    pub bitrate: u32,     // 比特率
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

// 解析音乐文件元数据
fn get_audio_metadata(index: u32, path: &PathBuf) -> Option<AudioMetadata> {
    if let Ok(tagged_file) = lofty::read_from_path(path) {
        let properties = tagged_file.properties();
        let tag = tagged_file.primary_tag();

        let duration = properties.duration().as_secs_f64();
        Some(AudioMetadata {
            index,
            path: path.display().to_string(),
            duration,
            duration_desc: format_duration_seconds(duration),
            sample_rate: properties.sample_rate().unwrap_or(0),
            channels: properties.channels().unwrap_or(0),
            bitrate: properties.audio_bitrate().unwrap_or(0),
            title: tag.and_then(|t| t.title().map(String::from)),
            artist: tag.and_then(|t| t.artist().map(String::from)),
            album: tag.and_then(|t| t.album().map(String::from)),
        })
    } else {
        None
    }
}

// 将因为时长转化为 分钟:秒数 格式
fn format_duration_seconds(seconds: f64) -> String {
    let minutes = (seconds / 60.0).floor() as i32;
    let seconds_remaining = (seconds % 60.0) as i32;

    format!("{:02}:{:02}", minutes, seconds_remaining)
}

// 打开文件夹对话框选择多个音频文件
fn get_audio_files() -> Vec<PathBuf> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(folder) = FileDialog::new().set_title("选择音乐文件夹").pick_folder() {
            let mut audio_files = Vec::new();

            // 扫描文件夹中的音频文件
            scan_audio_files(&folder, &mut audio_files);
            audio_files
        } else {
            Vec::new()
        }
    }
    // Web版本需要不同的实现
    #[cfg(target_arch = "wasm32")]
    {
        Vec::new()
    }
}

// 获取音乐文件元数据信息列表
pub fn get_audio_metadata_list() -> Vec<AudioMetadata> {
    let audio_files = get_audio_files();
    let mut audio_metadata_list: Vec<AudioMetadata> = Vec::new();
    for (index, audio_file) in audio_files.into_iter().enumerate() {
        if let Some(metadata) = get_audio_metadata(index as u32, &audio_file) {
            audio_metadata_list.push(metadata);
        }
    }
    audio_metadata_list
}

#[cfg(test)]
mod tests {
    use crate::load::loader::format_duration_seconds;

    #[test]
    fn test_format_duration_seconds() {
        let duration = 254.003f64;
        println!("{}", format_duration_seconds(duration));
        assert_eq!(format_duration_seconds(duration), "04:14");
    }
}
