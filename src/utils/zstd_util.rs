use std::fs::File;
use std::io::{Read, Write};
use tokio::fs;
use zstd::Encoder;

pub async fn compress_database() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut input_file: File = File::open("logDB.db")?;
    let mut input_data: Vec<u8> = Vec::new();

    input_file.read_to_end(&mut input_data)?;

    // let mut encoder: Encoder<File> = Encoder::new(File::create("database.zst")?, 1)?;
    //
    // encoder.write_all(&input_data)?;
    // encoder.finish()?;
    
    fs::remove_file("logDB.db").await.expect("Cannot remove database file");

    Ok(input_data)
}