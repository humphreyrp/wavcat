extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let mut verbose = false;
    let mut path = "".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("display a spectrogram view of the provided .wav file");
        ap.refer(&mut path).add_option(&["-f", "--filepath"], Store, "File to display").required();
        ap.refer(&mut verbose).add_option(&["-v","--verbose"], StoreTrue, "Be verbose");
        ap.parse_args_or_exit();
    }

    println!("Displaying wav file: {}", path);
}
