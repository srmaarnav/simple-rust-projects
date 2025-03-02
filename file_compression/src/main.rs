use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main(){
    if args().len() != 3 {
        println!("Usage: file_compression <source> <destination>");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!(
        "Source length: {:?} bytes", input.get_ref().metadata().unwrap().len() 
    );

    println!(
        "Compressed length: {:?} bytes", output.metadata().unwrap().len()
    );

    println!(
        "Elapsed time: {:?} seconds", start.elapsed().as_secs()
    );
}