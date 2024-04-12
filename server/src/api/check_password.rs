use axum::{http::{Response, StatusCode}, Json};
use serde::{Deserialize, Serialize};

use crate::util::check_password_strength;

use super::util::{generic_json_response, json_response};

#[derive(Deserialize)]
pub struct CheckPassInput {
	password: String,
	inputs: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct SerializableFeedback {
	warning: Option<String>,
	suggestions: Vec<String>,
}

impl From<&zxcvbn::feedback::Feedback> for SerializableFeedback {
	fn from(value: &zxcvbn::feedback::Feedback) -> Self {
		let warning =  match value.warning() {
			Some(warning) => Some(warning.to_string()),
			None => None,
		};

		let suggestions: Vec<String> = value.suggestions().into_iter().map(|suggestion| {
			suggestion.to_string()
		}).collect();
		
		Self {
			warning,
			suggestions,
		}
	}
}

#[derive(Serialize)]
pub struct CheckPassOutput {
	pub feedback: Option<SerializableFeedback>
}

pub async fn check_pass(
	Json(payload): Json<CheckPassInput>
) -> Response<axum::body::Body> {
	let strength = check_password_strength(&payload.password, payload.inputs);

	if strength.is_err() {
		return generic_json_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to check password.")
	}

	let strength = strength.unwrap();

	if strength.score() >= 3 {
		return generic_json_response(StatusCode::OK, "Password OK.")
	}

	let feedback = match strength.feedback() {
		Some(feedback) => Some(SerializableFeedback::from(feedback)),
		None => None
	};

	json_response(StatusCode::OK, CheckPassOutput {
		feedback
	})
}