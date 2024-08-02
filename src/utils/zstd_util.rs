use std::fs::File;
use std::io::{Read, Write};

use zstd::Encoder;

pub fn compress_database() -> Result<(), Box<dyn std::error::Error>> {
    let mut input_file: File = File::open("logDB.db")?;
    let mut input_data: Vec<u8> = Vec::new();

    input_file.read_to_end(&mut input_data)?;

    let mut encoder: Encoder<File> = Encoder::new(File::create("database.zst")?, 1)?;

    encoder.write_all(&input_data)?;
    encoder.finish()?;

    Ok(())
}