use crate::MusicInfo;

pub enum Command {
    SelectFiles,         // 选择歌曲
    Playback(MusicInfo), // 播放指定的歌曲, 是否停止已缓存的歌曲
    Play,                // 播放当前歌曲
    Pause,               // 暂停播放当前歌曲
    Prev,                // 播放上一首
    Next,                // 播放下一首
    ChangeProgress(f32), // 拖拽进度
    HopeRepeat(bool),    // 希望重复列表播放
    HopeShuffle(bool),   // 希望乱序播放
}
