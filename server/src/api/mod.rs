pub mod util;
pub mod user;
pub mod quiz;
pub mod question;
pub mod quiz_types;

use std::sync::Arc;

use axum::{routing::get, Router};

use crate::AppState;

use self::{question::question_router, quiz::quiz_router, user::user_router};

async fn root() -> &'static str {
	"Hello world"
}

pub fn api_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/", get(root))
		.nest("/user", user_router(Arc::clone(&state)))
		.nest("/quiz", quiz_router(Arc::clone(&state)))
		.nest("/question", question_router(Arc::clone(&state)))
}