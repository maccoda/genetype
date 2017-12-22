extern crate genetype;

use std::env;
use std::fs;
use std::io::Write;


use genetype::Converter;


fn main() {
    let path = env::args().nth(1).unwrap();
    println!("Converting {}", path);
    let buffer = Converter::new(path).convert().unwrap();

    // Write it to a file
    let output_path = "gene_out.rs";
    println!("Output written to {}", output_path);
    let mut out_file = fs::File::create(output_path).unwrap();
    let content: &[u8] = &(buffer.into_bytes())[..];
    out_file.write_all(content).unwrap();
}
