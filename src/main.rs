extern crate argparse;

use argparse::{ArgumentParser, Store, StoreTrue};
use image::{DynamicImage, Pixel, Rgba, RgbaImage};
use std::path::Path;
use std::process;
use viuer;
use wavers::Wav;

fn main() {
    let mut verbose = false;
    let mut fp: String = "".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("display a spectrogram view of the provided .wav file");
        ap.refer(&mut fp)
            .add_option(&["-f", "--filepath"], Store, "File to display")
            .required();
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.parse_args_or_exit();
    }

    // Verify path exists
    if !Path::new(&fp).exists() {
        println!("File does not exist");
        process::exit(1);
    }

    // Loop through the file in chunks
    let chunk_size = 16;
    let mut wav: Wav<i16> = Wav::from_path(&fp).unwrap();
    let _res = loop {
        // let _res = wav.read_samples(16);
        let res2 = wav.seek_by_samples(chunk_size);
        let _data = match res2 {
            Ok(data) => data,
            Err(_error) => break,
        };
    };

    let conf = viuer::Config {
        absolute_offset: false,
        ..Default::default()
    };

    let mut img = DynamicImage::ImageRgba8(RgbaImage::new(60, 60));
    let start = Rgba::from_slice(&[255, 0, 0, 255]);
    let end = Rgba::from_slice(&[0, 0, 255, 255]);
    image::imageops::vertical_gradient(&mut img, start, end);

    viuer::print(&img, &conf).unwrap();
}
