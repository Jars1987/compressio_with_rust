use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        println!("Please insert: `source` `target`");
        return
    }
    
    let source = args().nth(1).unwrap();
    let target = args().nth(2).unwrap();
    let inner_file = match File::open(source) {
        Ok(file) => file,
        Err(e) => {
            print!("Unable to open file! {:?}", e);
            return
        }
    };
    let mut input = BufReader::new(inner_file);

    //make another match here to check for errors
    let output = File::create(target).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();

    //make another match here to check for errors while finishing the encoding
    let output = encoder.finish().unwrap();

    println!("Source len: {:?}", input.get_ref().metadata().unwrap().len());
    println!("Target len: {:?}", output.metadata().unwrap().len());
    print!("Elapsed time: {:?}", start.elapsed())

}
