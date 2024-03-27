use std::sync::Arc;

use axum::{extract::State, http::{Response, StatusCode}, routing::get, Extension, Router};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, middleware::CurrentSession, util::generate_short_uuid};

use super::util::{generic_error, generic_response, json_response};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ReturnedPathId {
    pub id: String,
}

async fn root() -> &'static str {
	"Hello from files world"
}

async fn get_temp_path(
	Extension(current_session): Extension<CurrentSession>,
	State(state): State<Arc<AppState>>,
) -> Response<axum::body::Body> {
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }
	
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");

	json_response(StatusCode::OK, ReturnedPathId {
		id: generate_short_uuid().to_string(),
	})
}

pub fn files_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/", 
			get(root)
			.post(get_temp_path)
		)
		.with_state(state)
}