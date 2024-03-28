use async_trait::async_trait;


use super::file_storage_engine::FileStorageEngine;
use std::{env, fs::{self, OpenOptions}, io::Write};

#[derive(Clone)]
pub struct DiskStorage {
	pub file_folder: String,
}

impl DiskStorage {
	pub fn new() -> Self {
		Self {
			file_folder: env::var("FILE_STORAGE_PATH").expect("FILE_STORAGE_PATH must be set")
		}
	}

	fn folder_exists(&self, folder_name: &str) -> bool {
		fs::metadata(&folder_name).is_ok()
	}

	pub async fn prepare(&self) {
		if !self.folder_exists(&self.file_folder) {
			// Create the folder if it doesn't exist
			match fs::create_dir(&self.file_folder) {
				Ok(_) => println!("Folder '{}' created successfully!", self.file_folder),
				Err(e) => eprintln!("Error creating folder '{}': {}", self.file_folder, e),
			}
		}
	}
}


#[async_trait]
impl FileStorageEngine for DiskStorage {
	async fn upload_file(&self, data: &[u8], file_name: String) -> Result<String, std::io::Error> {
		self.prepare().await;

		let path = format!("{}/{}", self.file_folder, &file_name);
		
		// // Open a new file to save the uploaded content
		let mut output_file = OpenOptions::new()
			.write(true)
			.create_new(true)
			.open(path.clone())
			.unwrap();

		let _ = output_file.write_all(&data);
		Ok(path)
	}

	async fn serve_file(&self, file_id: String) -> Result<String, std::io::Error> {
		Ok("test".to_string())
	}
}