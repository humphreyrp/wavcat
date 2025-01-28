extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};
use wavers::Wav;
use std::path::Path;
use std::process;
use viuer::{print, Config};
use std::{thread, time};

fn main() {
    let mut verbose = false;
    let mut fp: String = "".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("display a spectrogram view of the provided .wav file");
        ap.refer(&mut fp).add_option(&["-f", "--filepath"], Store, "File to display").required();
        ap.refer(&mut verbose).add_option(&["-v","--verbose"], StoreTrue, "Be verbose");
        ap.parse_args_or_exit();
    }

    // Verify path exists
    if !Path::new(&fp).exists()
    {
        println!("File does not exist");
        process::exit(1);
    }

    // Load wav file
    let _wav: Wav<i16> = Wav::from_path(&fp).unwrap();

    // Start adding image framework
    let conf = Config {
        // Start from row 4 and column 20.
        x: 20,
        y: 4,
        ..Default::default()
    };

    let img = image::DynamicImage::ImageRgba8(image::RgbaImage::new(20, 10));
    print(&img, &conf).expect("Image printing failed.");

    thread::sleep(time::Duration::from_millis(1000));
}
