use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::Response, routing::post, Router};
use diesel::{ExpressionMethods, prelude::*};

use crate::{app_state::AppState, db::{models::EmailVerification, schema::{email_verification, users}}};

use super::util::{generic_error, generic_json_response};

async fn verify_email(
	Path(id): Path<String>,
	State(state): State<Arc<AppState>>,
) -> Response<axum::body::Body> {
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");

	let verification_row = email_verification::table.filter(
		email_verification::verification_token.eq(id)
	).first::<EmailVerification>(&mut conn);

	if verification_row.is_err() {
		return generic_error(StatusCode::NOT_FOUND, "Verification token not found.");
	}

	let verification_row: EmailVerification = verification_row.unwrap();

	let _ = diesel::update(users::table)
		.filter(users::id.eq(verification_row.user_id))
		.set(users::verified_email.eq(true))
		.execute(&mut conn);

	let _ = diesel::delete(email_verification::table)
		.filter(email_verification::id.eq(verification_row.id))
		.execute(&mut conn);

	generic_json_response(StatusCode::OK, "Email verified")
}



pub fn email_router(state: Arc<AppState>) -> Router {
	Router::new()
	.route(
		"/:id",
		post(verify_email)
	)
	.with_state(state)
}