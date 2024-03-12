pub mod user;

use axum::{routing::get, Router};

use self::user::user_router;

async fn root() -> &'static str {
	"Hello world"
}

pub fn api_router() -> Router {
	Router::new()
		.route("/", get(root))
		.nest("/user", user_router())
}