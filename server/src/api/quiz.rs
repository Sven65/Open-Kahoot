use axum::{extract::Path, http::{Response, StatusCode}, routing::get, Json, Router};
use diesel::{prelude::*, QueryDsl};
use tracing::info;


use crate::{api::util::{generic_error, generic_json_response, json_response}, db::{establish_connection, models::{Answer, Question, Quiz}, schema::{questions, quiz, users, answers}}, util::generate_short_uuid};

use super::quiz_types::ReturnedQuiz;


pub async fn get_quiz_by_id (quiz_id: String, conn: &mut PgConnection) -> Result<ReturnedQuiz, diesel::result::Error> {
	let quiz = quiz::table.find(quiz_id).first::<Quiz>(conn)?;
    let questions = Question::belonging_to(&quiz).load::<Question>(conn)?;
	let answers = Answer::belonging_to(&questions).load::<Answer>(conn)?;
	let owner = users::table
		.filter(users::id.eq(quiz.clone().owner_id))
		.select((users::id, users::username))
		.get_result::<(String, String)>(conn)?;


	Ok(ReturnedQuiz::new_from(quiz, questions, answers, owner))
}

async fn get_quiz(Path(id): Path<String>) -> Response<axum::body::Body> {
	let conn = &mut establish_connection();

	match get_quiz_by_id(id, conn).await {
		Ok(quiz) => json_response(StatusCode::OK, quiz),
		Err(e) => generic_error(StatusCode::NOT_FOUND, e.to_string().as_str())
	}
}

async fn update_quiz(
	Path(_id): Path<i32>,
	Json(new_quiz): Json<ReturnedQuiz>,
) -> Response<axum::body::Body> {
	info!("Payload is {:#?}", new_quiz);

	let mut conn = establish_connection();
	let cloned_quiz = new_quiz.clone();

	for mut ret_question in new_quiz.questions {
		let new_question_id = generate_short_uuid();

		if ret_question.id.is_none() {
			ret_question.id = Some(new_question_id.clone())
		}

		let update_question = Question::from(ret_question.clone());

		
		let _ = diesel::insert_into(crate::api::quiz::questions::dsl::questions)
			.values(&update_question)
			.on_conflict(questions::id)
			.do_update()
			.set(&update_question)
			.execute(&mut conn);

		for mut ret_answer in ret_question.answers {	
			if ret_answer.question_id.is_none() {
				ret_answer.question_id = Some(new_question_id.clone())
			}

			if ret_answer.id.is_none() {
				ret_answer.id = Some(generate_short_uuid())
			}

			let new_answer = Answer::from(ret_answer);

			let _ = diesel::insert_into(crate::api::quiz::answers::dsl::answers)
				.values(&new_answer)
				.on_conflict(answers::id)
				.do_update()
				.set(&new_answer)
				.execute(&mut conn);
		};
	};

	let _= diesel::update(crate::api::quiz::quiz::dsl::quiz)
		.filter(quiz::id.eq(cloned_quiz.id.clone().unwrap()))
		.set(Quiz::from(cloned_quiz))
		.execute(&mut conn);

		generic_json_response(StatusCode::OK, "Update OK")
}


async fn delete_quiz(Path(_id): Path<i32>) -> &'static str {
	"hello delete"
}

pub fn quiz_router() -> Router {
	Router::new()
		.route(
			"/:id",
			get(get_quiz)
			.put(update_quiz)
			.delete(delete_quiz)
		)

	}