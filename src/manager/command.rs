use crate::MusicInfo;

pub enum Command {
    SelectFiles,     // 选择歌曲
    Play(MusicInfo), // 播放当前歌曲
}
