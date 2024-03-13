pub mod util;
pub mod user;
pub mod quiz;
pub mod quiz_types;

use axum::{routing::get, Router};

use self::{quiz::quiz_router, user::user_router};

async fn root() -> &'static str {
	"Hello world"
}

pub fn api_router() -> Router {
	Router::new()
		.route("/", get(root))
		.nest("/user", user_router())
		.nest("/quiz", quiz_router())
}