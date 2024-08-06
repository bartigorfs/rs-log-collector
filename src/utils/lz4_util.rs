use crate::get_app_config;
use crate::models::app::AppConfig;
use lz4_flex::block::compress_prepend_size;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tokio::{fs, io};

fn read_file_to_bytes(path: &Path) -> io::Result<Vec<u8>> {
    let mut file: File = File::open(path)?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub async fn compress_database() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let config: &AppConfig = get_app_config().await;

    let input_data: Vec<u8>;

    let path: &Path = Path::new(&config.db_path);
    let buffer: Vec<u8> = read_file_to_bytes(path)?;
    let byte_slice: &[u8] = &buffer;

    input_data = compress_prepend_size(byte_slice);

    fs::remove_file(&config.db_path)
        .await
        .expect("Cannot remove database file");

    Ok(input_data)
}
