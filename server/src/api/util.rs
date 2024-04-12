use axum::http::{header::SET_COOKIE, Response, StatusCode};
use serde::Serialize;

use crate::util::check_password_strength;

use super::check_password::{CheckPassOutput, SerializableFeedback};

#[derive(Serialize)]
struct GenericError {
	pub error: String
}

#[derive(Serialize)]
struct GenericMessahe {
	pub message: String
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


pub fn json_response_with_cookie<T: Serialize>(status: StatusCode, data: T, cookie: &str) -> Response<axum::body::Body> {
	Response::builder()
		.status(status)
		.header("Content-Type", "application/json")
		.header(SET_COOKIE, cookie)
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

pub fn generic_json_response(status: StatusCode, message: &str) -> Response<axum::body::Body> {
	let g_message = GenericMessahe {
		message: message.to_string()
	};

	json_response(status, g_message)
}

pub fn api_check_pass(
	password: &str,
	inputs: Option<Vec<String>>,
) -> Result<(), Response<axum::body::Body>> {
	let password_check = check_password_strength(
		&password, 
		inputs,
	);

	if password_check.is_err() {
		return Err(generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to check password requirements."))
	}

	let password_check = password_check.unwrap();

	if password_check.score() < 3 {
		let feedback = match password_check.feedback() {
			Some(feedback) => Some(SerializableFeedback::from(feedback)),
			None => None
		};
	
		return Err(json_response(StatusCode::BAD_REQUEST, CheckPassOutput {
			feedback
		}));
	}

	Ok(())
}