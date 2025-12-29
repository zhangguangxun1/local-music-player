use lofty::file::TaggedFileExt;

// 这部分代码从 zeedle 拷贝, 后续会将图片生成到播放目录中无需每次重新生成
// 从歌曲文件中解析封面
fn parse_album_cover(path: &str) -> Option<(Vec<u8>, u32, u32)> {
    if let Ok(tagged) = lofty::read_from_path(path)
        && let Some(tag) = tagged.primary_tag()
        && let Some(picture) = tag.pictures().first()
        && let Ok(img) = image::load_from_memory(picture.data())
    {
        let rgba = img.into_rgba8();
        let (width, height) = rgba.dimensions();
        let buffer = rgba.into_vec();
        return Some((buffer, width, height));
    }
    None
}

// 从内存中实时读取当前歌曲的封面信息
pub fn get_album_cover(path: &str) -> slint::Image {
    if let Some(cover) = parse_album_cover(path) {
        let (buffer, width, height) = cover;
        let mut pixel_buffer = slint::SharedPixelBuffer::new(width, height);
        let pixel_buffer_data = pixel_buffer.make_mut_bytes();
        pixel_buffer_data.copy_from_slice(&buffer);
        slint::Image::from_rgba8(pixel_buffer)
    } else {
        slint::Image::default()
    }
}
