extern crate flate2;

// Standard library imports for file I/O, command line arguments
// and time stamping
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

// Flate2 imports for GzEncoder and Compression
use flate2::write::GzEncoder;
use flate2::Compression;

fn main() {
    // If the user did not input the correct amount of arguments,
    // print a usage message and return.
    if args().len() != 3 {
        eprintln!("Usage: <source file> <target file>");
        return;
    }

    // Since the user has input the correct amount of arguments,
    // we can attempt to get the source file.
    // We must unwrap the args() iterator to get the nth(1) argument,
    // then we must unwrap the result returned from the File::open()
    // function.
    let mut input = BufReader::new(
        File::open(
            args().nth(1).unwrap()
        ).unwrap()
    );

    // We will attempt to create an output file using the nth(2) arg
    // by unwrapping it. If this returns an Ok, we can unwrap the Result
    // returnedf from the File::create() function.
    let output = File::create(
        args().nth(2).unwrap()
    ).unwrap();

    // Create a new GzEncoder with the output file and the default
    // compression value.
    let mut encoder = GzEncoder::new(
        output, Compression::default()
    );

    // Start a timer and begin compressing the target into the target location.
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    // Print some information about original file size, new file size,
    // and time elapsed.
    println!("Source file length: {:?}", input.get_ref().metadata().unwrap().len());
    println!("Target file length: {:?}", output.metadata().unwrap().len());
    println!("Elapsed time: {:?}", start.elapsed());
}
