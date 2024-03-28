use async_trait::async_trait;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};

use super::file_storage_engine::FileStorageEngine;
use std::{env, str::FromStr};

#[derive(Clone)]
pub struct S3Storage {
	client: S3Client,
}

impl S3Storage {
	pub async fn new() -> Self {
		let region = Region::from_str(&env::var("AWS_REGION").expect("AWS_REGION must be set.")).unwrap();

		let client = match env::var("S3_ENDPOINT") {
			Ok(endpoint) => S3Client::new(Region::Custom { name: "custom-ep".to_string(), endpoint }),
			Err(_) => S3Client::new(region)
		};

		Self {
			client
		}
	}
}


#[async_trait]
impl FileStorageEngine for S3Storage {
	async fn upload_file(&self, data: &[u8], file_name: String) -> Result<String, std::io::Error> {		
		let request = PutObjectRequest {
			bucket: env::var("S3_BUCKET").expect("S3 bucket must be set.").to_string(),
			key: file_name.clone(),
			body: Some(data.to_vec().into()), // Convert the byte array into a readable stream
			..Default::default()
		};
	
		// Upload the file to S3
		match self.client.put_object(request).await {
			Ok(_) => println!("File uploaded successfully"),
			Err(err) => eprintln!("Error uploading file: {}", err),
		}

		Ok(file_name)
	}
}