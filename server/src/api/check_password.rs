use axum::{http::{Response, StatusCode}, Json};
use serde::Deserialize;

use crate::util::check_password_strength;

use super::util::generic_json_response;

#[derive(Deserialize)]
struct CheckPassInput {
	password: String,
	inputs: Option<Vec<String>>,
}

pub async fn check_pass(
	Json(payload): Json<CheckPassInput>
) -> Response<axum::body::Body> {
	let strength = check_password_strength(&payload.password, payload.inputs);

	if strength.is_err() {
		return generic_json_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to check password.")
	}

	let strength = strength.unwrap();

	

	generic_json_response(StatusCode::OK, "pass checked")
}