pub mod util;
pub mod user;
pub mod quiz;
pub mod question;
pub mod quiz_types;
pub mod files;
pub mod email;
pub mod check_password;

use std::sync::Arc;

use axum::{routing::{get, post}, Router};
use crate::AppState;

use self::{email::email_router, files::files_router, question::question_router, quiz::quiz_router, user::user_router};


async fn root() -> &'static str {
	"Hello world"
}


pub fn api_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/", get(root))
		.route("/check_password", post(check_password::check_pass))
		.nest("/email", email_router(Arc::clone(&state)))
		.nest("/user", user_router(Arc::clone(&state)))
		.nest("/quiz", quiz_router(Arc::clone(&state)))
		.nest("/question", question_router(Arc::clone(&state)))
		.nest("/files", files_router(Arc::clone(&state)))
}