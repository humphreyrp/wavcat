extern crate argparse;

use argparse::{ArgumentParser, Store, StoreTrue};
use image::{DynamicImage, Pixel, Rgba, RgbaImage};
use rustfft::{algorithm::Radix4, num_complex::Complex, Fft};
use std::path::Path;
use std::process;
use viuer;
use wavers::{Samples, Wav};

fn samples_to_buffer(samples: Samples<i16>) -> Vec<Complex<f32>> {
    let mut out = Vec::with_capacity(samples.len() / 2);
    for pair in samples.chunks_exact(2) {
        out.push(Complex::<f32>::new(pair[0] as f32, pair[1] as f32));
    }
    out
}

fn to_magnitude(samples : Vec<Complex<f32>>) -> Vec<f32> {
    let mut out = Vec::with_capacity(samples.len());
    for s in samples {
        out.push(s.norm())
    }
    out
}

fn handle_block(block: Samples<i16>, fft_size: usize) {
    println!("Calculating block of size: {}, length: {}", fft_size, block.len());
    // Drop the last block that is not the expected block size
    if block.len() / 2 != fft_size {
        return;
    }

    // Create an FFT instance
    let fft = Radix4::<f32>::new(fft_size, rustfft::FftDirection::Forward);

    // Convert to a vector of complex floats
    let mut buffer = samples_to_buffer(block);
    fft.process(&mut buffer);

    let _fft_mags = to_magnitude(buffer);
}

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

    // Setup the FFT
    let fft_size = 1024;

    // Loop through the file in chunks
    let mut wav: Wav<i16> = Wav::from_path(&fp).unwrap();
    for block in wav.blocks(fft_size, 0) {
        handle_block(block, fft_size);
    }

    // Display spectrogram
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
