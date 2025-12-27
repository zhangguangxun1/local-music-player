use crate::LyricInfo;
use slint::SharedString;
use std::fs;
use std::path::Path;

// 检查文件是否存在
fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

// 更简洁的方式：使用Path的方法
fn replace_ext_to_lrc(file_path: &str) -> String {
    Path::new(file_path)
        .with_extension("lrc") // 直接替换后缀
        .to_string_lossy()
        .to_string()
}

// 将歌曲文件归一到歌词文件, 文件同名, 后缀为 .lrc 的为该歌曲的歌词文件
fn get_lrc_file(file_path: &str) -> Option<String> {
    if !file_exists(file_path) {
        return None;
    }

    let lrc_file = replace_ext_to_lrc(file_path);
    if file_exists(&lrc_file) {
        Some(lrc_file)
    } else {
        None
    }
}

// 歌词结构体
pub struct LyricLine {
    pub time: u64,
    pub text: String,
}

// 将歌词文件解析为时间维度的歌词列表
fn parse_lyrics(lrc_file: &str) -> Vec<LyricLine> {
    let mut lines = Vec::new();
    let content = fs::read_to_string(lrc_file).expect("Failed to read file");
    if content.is_empty() {
        return lines;
    }

    for line in content.lines() {
        let mut duration: u64 = 0;
        if line.starts_with("[0") && line.len() > 10 {
            let minutes: u64 = line[1..3].parse().unwrap_or(0);
            let seconds: f64 = line[4..9].parse().unwrap_or(0.0);
            let total_ms: u64 = (minutes * 60 * 1000) + (seconds * 1000.0) as u64;

            duration = total_ms;

            let text = line[10..].trim().to_string();
            lines.push(LyricLine {
                time: duration,
                text,
            })
        } else if line.starts_with("[by:") {
            // 否则该一整行都看作一个普通的信息, 且时间为上一行的相同时间
            // [by:QQ音乐动态歌词]
            lines.push(LyricLine {
                time: duration,
                text: match get_by_content(&line.to_string().as_str()) {
                    Some(content) => content.to_string(),
                    None => "".to_string(),
                },
            })
        }
    }

    // 排序确保顺序正确
    lines.sort_by(|a, b| a.time.cmp(&b.time));

    lines
}

// 获取 [by:xxx] 中 by 后面的文字
fn get_by_content(text: &str) -> Option<&str> {
    // 查找 [by:
    if let Some(start) = text.find("[by:") {
        let start_pos = start + 4; // "[by:" 的长度是4

        // 查找对应的 ]
        if let Some(end) = text[start_pos..].find(']') {
            let end_pos = start_pos + end;
            return Some(&text[start_pos..end_pos]);
        }
    }
    None
}

// 获取歌曲对应的歌词列表
pub fn get_lyric_info_list(path: &str) -> Vec<LyricInfo> {
    let mut lyric_info_list: Vec<LyricInfo> = Vec::new();
    if let Some(lrc_file) = get_lrc_file(path) {
        let _lines = parse_lyrics(&lrc_file);
        for _line in _lines {
            lyric_info_list.push(LyricInfo {
                text: SharedString::from(_line.text),
                time: _line.time as f32,
            });
        }
    }

    lyric_info_list
}
