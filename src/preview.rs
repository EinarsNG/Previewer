use ffmpeg_next::frame::Video;

pub fn combine_images(
    frames: Vec<Video>,
    border_size: usize,
    scaling_factor: f32,
) -> image::DynamicImage {
    if frames.len() == 0 {
        return image::DynamicImage::new_rgb8(0, 0);
    }

    // case when sqrt from image count is not a whole number
    let image_count_sqrt = f32::sqrt(frames.len() as f32);
    let image_count_x = image_count_sqrt.floor() as u32;
    let image_count_y = image_count_sqrt.ceil() as u32;

    let total_width = image_count_x as u32 * frames[0].width();
    let total_height = image_count_y as u32 * frames[0].height();

    let mut img = image::ImageBuffer::new(
        total_width + border_size as u32,
        total_height + border_size as u32,
    );
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let x_index: u32 = x / frames[0].width();
        let y_index: u32 = y / frames[0].height();

        let frame_index = (y_index * image_count_x + x_index) as usize;
        if frame_index >= frames.len() {
            *pixel = image::Rgb([0, 0, 0]);
            continue;
        }
        let frame: &Video = &frames[frame_index];
        let data = frame.data(0);
        let stride = frame.stride(0);

        let yy = (y - y_index * frames[0].height()) as usize;
        let xx = (x - x_index * frames[0].width()) as usize;

        // black border
        if xx < border_size
            || yy < border_size
            || xx > frames[0].width() as usize + border_size
            || yy > frames[0].height() as usize + border_size
        {
            *pixel = image::Rgb([0, 0, 0]);
            continue;
        }

        let offset = (yy - border_size) * stride;

        *pixel = image::Rgb([
            data[offset + (xx - border_size) * 3],
            data[offset + (xx - border_size) * 3 + 1],
            data[offset + (xx - border_size) * 3 + 2],
        ]);
    }

    let dyn_image = image::DynamicImage::from(img);

    if scaling_factor != 1.0 {
        let width = (dyn_image.width() as f32 * scaling_factor).ceil() as u32;
        let height = (dyn_image.height() as f32 * scaling_factor).ceil() as u32;
        dyn_image.resize(width, height, image::imageops::Lanczos3)
    } else {
        dyn_image
    }
}
