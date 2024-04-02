pub mod util;
pub mod user;
pub mod quiz;
pub mod question;
pub mod quiz_types;
pub mod files;

use std::sync::Arc;

use axum::{routing::{get, post}, Router};
use tracing::info;

use crate::{email::Email, AppState};

use self::{files::files_router, question::question_router, quiz::quiz_router, user::user_router};

async fn root() -> &'static str {
	"Hello world"
}

async fn send_email() -> &'static str {
	let mail = Email::new().unwrap();

	let result = mail.send("Test Email", "thormax5@gmail.com", "Hello, this is a test email").await;

	if result.is_err() {
		info!("Error sending email {:#?}", result.err());
	}

	"Email sent"
}

pub fn api_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/", get(root))
		.route("/email", post(send_email))
		.nest("/user", user_router(Arc::clone(&state)))
		.nest("/quiz", quiz_router(Arc::clone(&state)))
		.nest("/question", question_router(Arc::clone(&state)))
		.nest("/files", files_router(Arc::clone(&state)))
}