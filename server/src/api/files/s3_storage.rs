use async_trait::async_trait;
use rusoto_core::credential::AwsCredentials;
use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};

use super::file_storage_engine::FileStorageEngine;
use std::{env, str::FromStr};

#[derive(Clone)]
pub struct S3Storage {
	client: S3Client,
	bucket: String,
	region: Region,
	credentials: AwsCredentials,
}

impl S3Storage {
	pub fn get_credentials() -> AwsCredentials {
		AwsCredentials::new(
			env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set."),
			env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set."),
			None, 
		None)
	}

	pub async fn new() -> Self {
		let endpoint_var = env::var("S3_ENDPOINT");

		let region = match endpoint_var {
			Ok(endpoint) => Region::Custom { name: "custom-ep".to_string(), endpoint },
			Err(_) => Region::from_str(&env::var("AWS_REGION").expect("AWS_REGION must be set.")).unwrap()
		};

		let client = S3Client::new(region.clone());

		Self {
			client,
			bucket: env::var("S3_BUCKET").expect("S3 bucket must be set.").to_string(),
			region,
			credentials: S3Storage::get_credentials(),
		}
	}
}


#[async_trait]
impl FileStorageEngine for S3Storage {
	async fn upload_file(&self, data: &[u8], file_name: String) -> Result<String, std::io::Error> {		
		let request = PutObjectRequest {
			bucket: self.bucket.clone(),
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

	async fn serve_file(&self, file_name: String) -> Result<String, std::io::Error> {
		let request = GetObjectRequest {
			bucket: self.bucket.clone(),
			key: file_name,
			..Default::default()
		};

		// Generate a pre-signed URL for the object
		let presigned_url = request.get_presigned_url(&self.region, &self.credentials, &PreSignedRequestOption::default());


		Ok(presigned_url)
	}

	fn get_file_path(&self) -> String {
		panic!("Not implemented");
	}
}