use axum::{extract::Path, http::{Response, StatusCode}, routing::delete, Extension, Router};
use diesel::prelude::*;

use crate::{db::{establish_connection, models::Question, schema::questions}, middleware::CurrentSession};

use super::{quiz::get_quiz_by_id, util::{generic_error, generic_json_response}};


/// Gets a question by it's ID
/// 
/// * question_id The ID of the question to get
pub async fn get_question_by_id (question_id: String, conn: &mut PgConnection) -> Result<Question, diesel::result::Error> {
	let returned_question = questions::table.find(question_id).first::<Question>(conn)?;

	Ok(returned_question)
}

async fn delete_question(
	Path(id): Path<String>,
	Extension(current_session): Extension<CurrentSession>,
) -> Response<axum::body::Body> {	
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }
	
	let mut conn = establish_connection();

	let db_question = get_question_by_id(id.clone(), &mut conn).await;

	if db_question.is_err() { return generic_error(StatusCode::NOT_FOUND, "Question not found."); }
	let db_question = db_question.unwrap();

	let quiz = get_quiz_by_id(db_question.quiz_id, &mut conn).await;

	if quiz.is_err() { return generic_error(StatusCode::NOT_FOUND, "Questions quiz not found."); }
	let quiz = quiz.unwrap();

	if !current_session.match_user_id(quiz.owner.id) { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }

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