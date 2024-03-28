use async_trait::async_trait;
use std::io::Error;

#[async_trait]
pub trait FileStorageEngine {
	async fn upload_file(&self, data: &[u8], file_name: String) -> Result<String, Error>; 
	async fn serve_file(&self, file_id: String) -> Result<String, Error>;
}