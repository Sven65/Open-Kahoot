use std::env;

use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

pub struct Email {
	mailer: AsyncSmtpTransport<Tokio1Executor>,
	from: String,
}

impl Email {
	pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
		let smtp_credentials= Credentials::new(
			env::var("SMTP_USERNAME").expect("Expected SMTP_USERNAME to be set"),
			env::var("SMTP_PASSWORD").expect("Expected SMTP_PASSWORD to be set")
		);

		let smtp_host = env::var("SMTP_HOST").expect("Expected SMTP_HOST to be set.");

		let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host.as_str())?
		.credentials(smtp_credentials)
		.build();
		
		Ok(Self {
			mailer,
			from: env::var("SMTP_FROM").expect("Expected SMTP_FROM to be set"),
		})
	}

	pub async fn send(&self, subject: &str, to: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
		let email = Message::builder()
		.from(self.from.parse()?)
		.to(to.parse()?)
		.subject(subject)
		.body(body.to_string())?;

		self.mailer.send(email).await?;
		
		Ok(())
	}
}
