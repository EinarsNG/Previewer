mod preview;
mod structs;
mod utils;
mod video;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = utils::parse_args_or_exit();

    println!("Extracting video frames...");
    let frames = video::extract_preview_frames(args.video_path, args.image_count)?;

    println!("Creating the preview...");
    preview::combine_images(frames, args.border_size as usize, args.scaling_factor)
        .save(args.image_path)?;

    Ok(())
}
