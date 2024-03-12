use axum::http::{Response, StatusCode};
use serde::Serialize;

#[derive(Serialize)]
struct GenericError {
	pub error: String
}

#[allow(dead_code)]
pub fn generic_response(status: StatusCode, message: &str) -> Response<axum::body::Body> {
    Response::builder()
        .status(status)
        .body(axum::body::Body::from(message.to_string()))
        .expect("Failed to build response")
}

pub fn json_response<T: Serialize>(status: StatusCode, data: T) -> Response<axum::body::Body> {
	Response::builder()
		.status(status)
		.header("Content-Type", "application/json")
		.body(axum::body::Body::from(
			serde_json::to_string(&data)
				.expect("Failed to serialize data")
		))
		.expect("Failed to build response")
}

pub fn generic_error(status: StatusCode, message: &str) -> Response<axum::body::Body> {
	let error = GenericError {
		error: message.to_string()
	};

	json_response(status, error)
}