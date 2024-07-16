use std::fs::File;
use std::io::{Read, Write};

use zstd::Encoder;

pub fn compress_database() -> Result<(), Box<dyn std::error::Error>> {
    // Open the input file
    let mut input_file = File::open("database.hpdb")?;
    let mut input_data = Vec::new();

    // Read the input file into a vector
    input_file.read_to_end(&mut input_data)?;

    // Create a ZSTD encoder
    let mut encoder = Encoder::new(File::create("database.zst")?, 1)?; // Compression level 1

    // Compress the input data
    encoder.write_all(&input_data)?;
    encoder.finish()?;

    Ok(())
}