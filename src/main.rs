extern crate argparse;

use argparse::{ArgumentParser, Store, StoreTrue};
use image::{self, GrayImage, Luma};
use rustfft::{algorithm::Radix4, num_complex::Complex, Fft};
use core::f32;
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

fn to_db(samples : Vec<Complex<f32>>) -> Vec<f32> {
    let mut out = Vec::with_capacity(samples.len());
    for s in samples {
        out.push(20.0 * (s.norm() / 1.0).log10())
    }
    out
}

fn handle_block(block: Samples<i16>, fft_size: usize, out_frames: &mut Vec<Vec<f32>>) {
    // Drop the last block that is not the expected block size
    if block.len() / 2 != fft_size {
        return;
    }

    // Create an FFT instance
    let fft = Radix4::<f32>::new(fft_size, rustfft::FftDirection::Forward);

    // Convert to a vector of complex floats
    let mut buffer = samples_to_buffer(block);
    fft.process(&mut buffer);

    let fft_mags = to_db(buffer);
    out_frames.push(fft_mags);
}

fn quantize(val : f32, min : f32, max : f32) -> u8 {
    (((max - val) / (max - min)) * 255.0) as u8
}

fn main() {
    let mut verbose = false;
    let mut fft_size = 1024;
    let mut fp: String = "".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("display a spectrogram view of the provided .wav file");
        ap.refer(&mut fp)
            .add_option(&["-f", "--filepath"], Store, "File to display")
            .required();
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut fft_size).add_option(&["--fft-size"], Store, "FFT size");
        ap.parse_args_or_exit();
    }

    // Verify path exists
    if !Path::new(&fp).exists() {
        println!("File does not exist");
        process::exit(1);
    }

    // Output frames are a list of vectors
    let mut frames : Vec<Vec<f32>> = Vec::new();

    // Loop through the file in chunks
    let mut wav: Wav<i16> = Wav::from_path(&fp).unwrap();
    for block in wav.blocks(fft_size, 0) {
        handle_block(block, fft_size, &mut frames);
    }

    // Display spectrogram
    let conf = viuer::Config {
        absolute_offset: false,
        ..Default::default()
    };

    // Find the max and min values in the frames
    let mut max = -f32::INFINITY;
    let mut min = f32::INFINITY;
    for row in frames.iter() {
        // println!("{:?}", row);
        for val in row.iter() {
            if *val < min {
                min = *val;
            }
            if *val > max {
                max = *val;
            }
        }
    }

    println!("Min: {}, max: {}", min, max);

    let width = fft_size as u32;
    let height = frames.len() as u32;
    let mut img = GrayImage::new(width, height);
    for (y, row) in frames.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let v = quantize(val, min, max);
            // println!("val: {}, quantized: {}", val, v);
            img.put_pixel(x as u32, y as u32, Luma([v]));
        }
    }

    viuer::print(&image::DynamicImage::ImageLuma8(img), &conf).unwrap();
}
