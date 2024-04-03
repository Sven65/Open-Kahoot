use std::env;

use chrono::Duration;

#[derive(Clone)]
pub struct AppConfig {
	pub smtp_enabled: bool,
	pub enable_email_verification: bool,
	pub frontend_url: String,
	pub password_reset_request_time: Option<Duration>,
	pub password_reset_valid_time: Option<Duration>,
}

fn check_env_var(name: &str, error_msg: &str) -> Result<(), String> {
	let var = env::var(name);
	if var.is_err() {
		Err(error_msg.into())
	} else {
		if var.unwrap().is_empty() { return Err(error_msg.into()); }
		Ok(())
	}
}

fn check_bool_env_var(name: &str, error_msg: &str, expect_bool: bool) -> Result<(), String> {
	let var = match env::var(name) {
		Ok(val) => val,
		Err(_) => return Err(error_msg.into()),
	};

	if var.is_empty() {
		return Err(error_msg.into());
	}

	let var_bool = match var.to_lowercase().as_str() {
		"true" => true,
		"false" => false,
		_ => return Err(format!("Expected a boolean value for {}", name)),
	};

	if var_bool != expect_bool {
		Err(error_msg.into())
	} else {
		Ok(())
	}
}

impl AppConfig {
	fn validate_smtp_configuration(&self) -> Result<(), String> {
		if !self.smtp_enabled {
			return Ok(())
		}
		
		check_env_var("SMTP_USERNAME", "SMTP is enabled but SMTP_USERNAME is not set")?;
		check_env_var("SMTP_PASSWORD", "SMTP is enabled but SMTP_PASSWORD is not set")?;
		check_env_var("SMTP_FROM", "SMTP is enabled but SMTP_FROM is not set")?;
		check_env_var("SMTP_HOST", "SMTP is enabled but SMTP_HOST is not set")?;

		return Ok(())
	}

	fn validate(&self) -> Result<(), String> {
		check_env_var("DATABASE_URL", "DATABASE_URL is not set")?;

		let file_storage_engine = env::var("FILE_STORAGE_ENGINE")
			.map_err(|_| "FILE_STORAGE_ENGINE is not set".to_string())?;
		
		if file_storage_engine != "disk" && file_storage_engine != "s3" {
			return Err("FILE_STORAGE_ENGINE must be set to either 'disk' or 's3'".to_string());
		}

		if file_storage_engine == "disk" {
			check_env_var("FILE_STORAGE_PATH", "FILE_STORAGE_PATH is required when FILE_STORAGE_ENGINE is set to 'disk'")?;
		}

		if file_storage_engine == "s3" {
			check_env_var("AWS_ACCESS_KEY_ID", "AWS_ACCESS_KEY_ID is required when FILE_STORAGE_ENGINE is set to 's3'")?;
			check_env_var("AWS_SECRET_ACCESS_KEY", "AWS_SECRET_ACCESS_KEY is required when FILE_STORAGE_ENGINE is set to 's3'")?;
			check_env_var("AWS_REGION", "AWS_REGION is required when FILE_STORAGE_ENGINE is set to 's3'")?;
			check_env_var("S3_BUCKET", "S3_BUCKET is required when FILE_STORAGE_ENGINE is set to 's3'")?;
			check_env_var("S3_ENDPOINT", "S3_ENDPOINT is required when FILE_STORAGE_ENGINE is set to 's3'")?;
		}
		
		check_env_var("FRONTEND_URL", "FRONTEND_URL is not set")?;

		if self.enable_email_verification {
			check_bool_env_var("SMTP_ENABLED", "SMTP_ENABLED must be set to true if ENABLE_EMAIL_VERIFICATION is set to true", true)?;
		}

		if let Some(password_reset_time) = self.password_reset_request_time {
			let enable_email_verification = self.enable_email_verification;
			if !password_reset_time.is_zero() && !enable_email_verification {
				return Err("PASSWORD_RESET_TIME is set to true but ENABLE_EMAIL_VERIFICATION is not set to true".to_string());
			}
		}

		if let Some(password_reset_valid_time) = self.password_reset_valid_time {
			let enable_email_verification = self.enable_email_verification;
			if !password_reset_valid_time.is_zero() && !enable_email_verification {
				return Err("PASSWORD_RESET_VALID_TIME is set to true but ENABLE_EMAIL_VERIFICATION is not set to true".to_string());
			}
		}
		
		if self.smtp_enabled {
			self.validate_smtp_configuration()?;
		}

		Ok(())
	}

	pub fn load_from_env() -> Self {
		let smtp_enabled = env::var("SMTP_ENABLED").map(|val| val == "true").unwrap_or(false);

		let enable_email_verification = env::var("ENABLE_EMAIL_VERIFICATION").map(|val| val == "true").unwrap_or(false);

		let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| String::new());

		let password_reset_request_time = match env::var("PASSWORD_RESET_REQUEST_TIME") {
			Ok(val) => {
				match parse_duration::parse(val.as_str()) {
					Ok(duration) => Some(Duration::from_std(duration).unwrap()), // Assuming no error for simplicity
					Err(_) => None,
				}
			}
			Err(_) => None,
		};
	
		println!("password_reset_request_time {:#?}", password_reset_request_time);

		let password_reset_valid_time = match env::var("PASSWORD_RESET_VALID_TIME") {
			Ok(val) => {
				match parse_duration::parse(val.as_str()) {
					Ok(duration) => Some(Duration::from_std(duration).unwrap()), // Assuming no error for simplicity
					Err(_) => None,
				}
			}
			Err(_) => None,
		};

		let config = Self {
			smtp_enabled,
			enable_email_verification,
			frontend_url,
			password_reset_request_time,
			password_reset_valid_time,
		};

		config.validate().unwrap();

		config
	}
}

