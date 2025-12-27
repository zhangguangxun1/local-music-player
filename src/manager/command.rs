use crate::{MusicInfo, PlayMode};

pub enum Command {
    SelectFiles,               // 选择歌曲
    Play(PlayMode, MusicInfo), // 播放当前歌曲
    PlayCurrent,               // 播放当前歌曲
    Pause,                     // 暂停播放当前歌曲
    Prev,                      // 播放上一首
    Next,                      // 播放下一首
    ChangeProgress(f32),       // 拖拽进度
    Repeat,                    // 重复列表播放
    Shuffle,                   // 乱序播放
}
