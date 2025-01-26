extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::path::Path;
use std::process;

fn main() {
    let mut verbose = false;
    let mut fp = "".to_string();

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

    println!("Displaying wav file: {}", fp);
}
