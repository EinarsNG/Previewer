use std::error::Error;

use ffmpeg_next::format::{self, Pixel};
use ffmpeg_next::frame::Video;
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
use ffmpeg_next::{rescale, Rescale};

use super::utils::linspace;

pub fn extract_preview_frames(
    video_path: String,
    preview_count: u32,
) -> Result<Vec<Video>, Box<dyn Error>> {
    let (video_frames, frame_rate) = probe_video(&video_path)?;

    let mut ictx = format::input(&video_path)?;
    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg_next::Error::StreamNotFound)?;

    let video_stream_index = input.index();

    let context_decoder =
        ffmpeg_next::codec::context::Context::from_parameters(input.parameters())?;

    let mut decoder = context_decoder.decoder().video()?;

    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;

    let mut frames_to_use = linspace(0, video_frames, preview_count + 2);
    // Remove first and last frames
    frames_to_use.pop_front();
    frames_to_use.pop_back();

    let mut frames: Vec<Video> = Vec::new();
    //let mut frame_index = 0;
    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg_next::decoder::Video| -> Result<bool, ffmpeg_next::Error> {
            let mut decoded = Video::empty();
            let mut iterations = 0;
            while decoder.receive_frame(&mut decoded).is_ok() {
                iterations += 1;
                let mut rgb_frame = Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;
                //save_file_jpg(&rgb_frame, frame_index);
                //frame_index += 1;
                frames.push(rgb_frame);
            }
            if iterations > 0 {
                decoder.flush();
            }
            Ok(iterations == 0)
        };

    loop {
        let mut framepos = (match frames_to_use.pop_front() {
            Some(res) => res,
            None => break,
        } as f32
            / frame_rate) as i64;
        framepos = framepos.rescale((1, 1), rescale::TIME_BASE);
        ictx.seek(framepos, framepos - 10..framepos)?;
        loop {
            // in case stream is not video we loop until it is
            if let Some((stream, packet)) = ictx.packets().next() {
                if stream.index() != video_stream_index {
                    continue;
                }
                decoder.send_packet(&packet)?;
                let should_continue = receive_and_process_decoded_frames(&mut decoder)?;
                // if we don't decode a frame after the first packet we need another one
                if should_continue {
                    continue;
                }
            }
            break;
        }
    }
    decoder.send_eof()?;
    Ok(frames)
}

#[allow(unused)]
fn save_file_jpg(frame: &Video, index: usize) -> Result<(), Box<dyn Error>> {
    let mut img = image::ImageBuffer::new(frame.width(), frame.height());
    let data = frame.data(0);
    let stride = frame.stride(0);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let yy = y as usize;
        let xx = x as usize * 3;
        *pixel = image::Rgb([
            data[yy * stride + xx],
            data[yy * stride + xx + 1],
            data[yy * stride + xx + 2],
        ])
    }
    img.save(format!("out/frame{}.jpg", index))?;

    Ok(())
}

fn probe_video(video_path: &String) -> Result<(u32, f32), ffmpeg_next::Error> {
    let mut video_frames = 0u32;
    let mut frame_rate = 0f32;
    {
        let mut probe = format::input(&video_path)?;
        let input = probe
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg_next::Error::StreamNotFound)?;

        let video_stream_index = input.index();

        for (stream, _) in probe.packets() {
            if stream.index() == video_stream_index {
                video_frames += 1;
                if frame_rate == 0.0 {
                    frame_rate =
                        stream.rate().numerator() as f32 / stream.rate().denominator() as f32;
                }
            }
        }
    }

    Ok((video_frames, frame_rate))
}
