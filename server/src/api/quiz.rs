use std::sync::Arc;

use axum::{extract::{Path, State}, http::{Response, StatusCode}, routing::{get, post}, Extension, Json, Router};
use diesel::{associations::HasTable, prelude::*, QueryDsl};
use serde::Deserialize;
use tracing::info;


use crate::{api::util::{generic_error, generic_json_response, json_response}, app_state::PgPooledConn, db::{models::{Answer, Files, Question, Quiz}, schema::{answers, files, questions, quiz, users}}, middleware::CurrentSession, util::generate_short_uuid, AppState};

use super::quiz_types::ReturnedQuiz;

#[derive(Deserialize)]
struct InCreatedQuiz {
	pub name: String,
}

pub async fn get_quiz_by_id (quiz_id: String, conn: &mut PgPooledConn) -> Result<ReturnedQuiz, diesel::result::Error> {
	let quiz = quiz::table.find(quiz_id).first::<Quiz>(conn)?;
    let questions: Vec<Question> = Question::belonging_to(&quiz).load::<Question>(conn)?;
	let answers = Answer::belonging_to(&questions).load::<Answer>(conn)?;
	let owner = users::table
		.filter(users::id.eq(quiz.clone().owner_id))
		.select((users::id, users::username))
		.get_result::<(String, String)>(conn)?;

	let files = questions.clone().iter().filter_map(|question| {
		let file: Result<Files, diesel::result::Error> = files::table.filter(files::question_id.eq(&question.id)).select(files::table::all_columns()).first::<Files>(conn);

		if file.is_ok() {
			return Some(file.unwrap());
		}

		None
	}).collect::<Vec<Files>>();

	Ok(ReturnedQuiz::new_from(quiz, questions, answers, owner, files))
}


async fn get_quiz(
	Path(id): Path<String>,
	Extension(current_session): Extension<CurrentSession>,
	State(state): State<Arc<AppState>>,
) -> Response<axum::body::Body> {
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }

	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");

	match get_quiz_by_id(id, &mut conn).await {
		Ok(quiz) => {
			if !quiz.public {
				if !current_session.match_user_id(quiz.owner.id.clone()) {
					return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized.");
				}
			}
			
			json_response(StatusCode::OK, quiz)
		},
		Err(e) => generic_error(StatusCode::NOT_FOUND, e.to_string().as_str())
	}
}

async fn update_quiz(
	Path(_id): Path<String>,
	Extension(current_session): Extension<CurrentSession>,
	State(state): State<Arc<AppState>>,
	Json(new_quiz): Json<ReturnedQuiz>,
) -> Response<axum::body::Body> {
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }
	
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");
	let cloned_quiz = new_quiz.clone();

	let db_quiz = get_quiz_by_id(cloned_quiz.clone().id.unwrap(), &mut conn).await;

	if db_quiz.is_err() { return generic_error(StatusCode::NOT_FOUND, "Quiz to update not found."); }

	if db_quiz.unwrap().owner.id != current_session.session.unwrap().user_id { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized.");  }

	for mut ret_question in new_quiz.questions {
		let new_question_id = generate_short_uuid();

		if ret_question.id.is_none() || ret_question.id.clone().unwrap().is_empty() {
			ret_question.id = Some(new_question_id.clone())
		}

		if ret_question.image_id.is_some() && !ret_question.image_id.clone().unwrap().is_empty() {
			let image_id = ret_question.image_id.clone().unwrap();

			let _ = diesel::update(crate::api::quiz::files::dsl::files)
				.filter(crate::api::quiz::files::id.eq(image_id))
				.set(crate::api::quiz::files::question_id.eq(ret_question.clone().id))
				.execute(&mut conn);
		}

		let update_question = Question::from(ret_question.clone());
		
		let _ = diesel::insert_into(crate::api::quiz::questions::dsl::questions)
			.values(&update_question)
			.on_conflict(questions::id)
			.do_update()
			.set(&update_question)
			.execute(&mut conn);

		if let Some(answers) = ret_question.answers {
			for mut ret_answer in answers {
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
		}
	};

	let _= diesel::update(crate::api::quiz::quiz::dsl::quiz)
		.filter(quiz::id.eq(cloned_quiz.id.clone().unwrap()))
		.set(Quiz::from(cloned_quiz))
		.execute(&mut conn);

		generic_json_response(StatusCode::OK, "Update OK")
}

async fn create_quiz(
	Extension(current_session): Extension<CurrentSession>,
	State(state): State<Arc<AppState>>,
	Json(new_quiz): Json<InCreatedQuiz>,
) -> Response<axum::body::Body> {
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }

	let current_session = current_session.session.unwrap();
	
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");
	let new_quiz_id = generate_short_uuid();

	let result = diesel::insert_into(crate::api::quiz::quiz::dsl::quiz)
		.values(Quiz::new(new_quiz_id, current_session.user_id, new_quiz.name))
		.returning(Quiz::as_returning())
		.get_result(&mut conn);

	match result {
		Ok(result) => json_response(StatusCode::CREATED, result),
		Err(e) => {
			info!("Quiz creation failed: {:#?}", e);
			generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create quiz.")
		}
	}
}

async fn delete_quiz(
	Path(id): Path<String>,
	Extension(current_session): Extension<CurrentSession>,
	State(state): State<Arc<AppState>>,
) ->  Response<axum::body::Body> {
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");

	match get_quiz_by_id(id.clone(), &mut conn).await {
		Ok(quiz) => {
			if !current_session.match_user_id(quiz.owner.id) { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }

			match diesel::delete(crate::api::quiz::quiz::dsl::quiz)
				.filter(quiz::id.eq(id))
				.execute(&mut conn) {
					Ok(_) => {
						info!("deleted");
						return generic_json_response(StatusCode::OK, "Deleted.");
					},
					Err(_) => generic_json_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete quiz.")
				}
		},
		Err(_) => generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete quiz.")
	}
}

pub fn quiz_router(state: Arc<AppState>) -> Router {
	Router::new()
	.route(
		"/:id",
		get(get_quiz)
		.put(update_quiz)
		.delete(delete_quiz)
	)
	.route("/create", post(create_quiz))
	.with_state(state)
}