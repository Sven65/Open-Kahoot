use axum::{extract::Path, http::{Response, StatusCode}, routing::get, Router};
use diesel::{prelude::*, QueryDsl};

use crate::{api::util::{generic_error, json_response}, db::{establish_connection, models::Quiz, schema::quiz}};

async fn get_quiz_by_id (quiz_id: i32, conn: &mut PgConnection) -> Result<Quiz, diesel::result::Error> {
	quiz::table
		.filter(quiz::id.eq(quiz_id))
		.select(Quiz::as_select())
		.get_result(conn)
}

async fn get_quiz(Path(id): Path<i32>) -> Response<axum::body::Body> {
	let conn = &mut establish_connection();

	match get_quiz_by_id(id, conn).await {
		Ok(quiz) => json_response(StatusCode::OK, quiz),
		Err(e) => generic_error(StatusCode::NOT_FOUND, e.to_string().as_str())
	}
}

pub fn quiz_router() -> Router {
	Router::new()
		.route("/:id", get(get_quiz))
}