use super::structs::Args;
use argparse::{ArgumentParser, Store};
use std::collections::VecDeque;

pub fn linspace(min: u32, max: u32, count: u32) -> VecDeque<u32> {
    match count {
        1 => return vec![min + (max - min) / 2].into(),
        0 => return vec![].into(),
        _ => {}
    }
    let step = (max - min) / (count - 1);
    let mut res = VecDeque::new();
    res.push_back(min);
    for i in 1..count - 1 {
        res.push_back(min + step * i);
    }
    res.push_back(max);
    res
}

pub fn parse_args_or_exit() -> Args {
    let mut video_path: String = Default::default();
    let mut scaling_factor: f32 = 1f32;
    let mut border_size: u32 = 6;
    let mut image_path: String = "preview.jpg".to_string();
    let mut image_count: u32 = 16;
    {
        let mut args = ArgumentParser::new();

        args.refer(&mut video_path)
            .add_option(&["-i", "--input"], Store, "Input video path.")
            .required();

        args.refer(&mut scaling_factor).add_option(
            &["-s", "--scaling"],
            Store,
            "Scaling factor (default: 1, which combines all images without scaling. Set it below 1 to downscale.).",
        );

        args.refer(&mut border_size).add_option(
            &["-b", "--border"],
            Store,
            "Border size (default: 6 px).",
        );

        args.refer(&mut image_path).add_option(
            &["-o", "--output"],
            Store,
            "Output image path (default: preview.jpg).",
        );

        args.refer(&mut image_count).add_option(
            &["-c", "--count"],
            Store,
            "Image count in the preview (default: 16).",
        );

        args.parse_args_or_exit();
    }

    if scaling_factor > 1.0 {
        println!("Warning: Setting scaling factor above 1.0");
    }

    Args {
        video_path,
        scaling_factor,
        border_size,
        image_path,
        image_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linspace_test() {
        let actual = linspace(0, 100, 6);
        let expected = vec![0, 20, 40, 60, 80, 100];
        assert_eq!(actual, expected);
    }

    #[test]
    fn linspace_test_2() {
        let actual = linspace(0, 100, 1);
        let expected = vec![50];
        assert_eq!(actual, expected);
    }

    #[test]
    fn linspace_test_3() {
        let actual = linspace(0, 100, 0);
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
