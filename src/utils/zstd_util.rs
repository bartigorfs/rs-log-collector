// use std::fs::File;
// use std::io::Read;
//
// use tokio::fs;
//
// use crate::get_app_config;
// use crate::models::app::AppConfig;
//
// pub async fn compress_database() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//     let config: &AppConfig = get_app_config().await;
//
//     let mut input_file: File = File::open(&config.db_path)?;
//     let mut input_data: Vec<u8> = Vec::new();
//
//     input_file.read_to_end(&mut input_data)?;
//
//     fs::remove_file(&config.db_path).await.expect("Cannot remove database file");
//
//     Ok(input_data)
// }