use async_trait::async_trait;
use aws_sdk_s3::Client;
use tracing::info;
use std::net::ToSocketAddrs;

use super::file_storage_engine::FileStorageEngine;
use std::env;

#[derive(Clone)]
pub struct S3Storage {
	client: Client,
}

impl S3Storage {
	pub async fn new() -> Self {
		let aws_configuration = aws_config::load_from_env().await;
		
		let aws_configuration = match env::var("S3_ENDPOINT") {
			Ok(endpoint) => {
				println!("endpoint is {}", endpoint);
				aws_configuration
				.to_builder()
				.endpoint_url(endpoint)
				.build()
			}
			Err(_) => aws_configuration,
		};
		
		info!("endpoint url {:#?}", aws_configuration.endpoint_url());

		//create aws s3 client
		let client = Client::new(&aws_configuration);

		Self {
			client,
		}
	}
}


#[async_trait]
impl FileStorageEngine for S3Storage {
	async fn upload_file(&self, data: &[u8], file_name: String) -> Result<String, std::io::Error> {		
		let res = self.client.put_object()
			.key(file_name)
			.bucket(env::var("AWS_S3_BUCKET").expect("Expected S3 bucket to be set."))
			.body(data.to_vec().into())
			.send()
			.await;


		if res.is_err() {
			println!("Failed to upload {:#?}", res.err());
		}
		
		Ok("s3 engine".to_string())
	}
}