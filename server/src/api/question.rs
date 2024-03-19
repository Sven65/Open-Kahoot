use axum::{extract::Path, http::{Response, StatusCode}, routing::delete, Router};
use diesel::prelude::*;

use crate::db::{establish_connection, schema::questions};

use super::util::generic_json_response;


async fn delete_question(Path(id): Path<String>) -> Response<axum::body::Body> {
	let mut conn = establish_connection();


	match diesel::delete(crate::api::question::questions::dsl::questions)
		.filter(questions::id.eq(id))
		.execute(&mut conn) {
			Ok(_) => generic_json_response(StatusCode::NO_CONTENT, "Deleted."),
			Err(e) => generic_json_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string())
		}
}

pub fn question_router() -> Router {
	Router::new()
	.route(
		"/:id",
		delete(delete_question)
	)
}