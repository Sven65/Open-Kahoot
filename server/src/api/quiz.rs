use axum::{extract::Path, http::{Response, StatusCode}, routing::get, Router};
use chrono::NaiveDateTime;
use diesel::{prelude::*, QueryDsl};

use crate::{api::util::{generic_error, json_response}, db::{establish_connection, models::{Answer, Question, Quiz}, schema::{quiz, users}}};

#[derive(Debug, Clone, serde::Serialize)]
struct ReturnedQuestion {
	id: i32,
	quiz_id: i32,
	question: String,
	created_at: NaiveDateTime,
	updated_at: NaiveDateTime,
	answers: Vec<Answer>,
	question_rank: i32,
}

#[derive(Debug, Clone, serde::Serialize)]
struct ReturnedUser {
	id: i32,
	username: String,
}
#[derive(Debug, Clone, serde::Serialize)]
struct ReturnedQuiz {
	id: i32,
	owner: ReturnedUser,
	name: String,
	created_at: NaiveDateTime,
	updated_at: NaiveDateTime,
	questions: Vec<ReturnedQuestion>,
}

impl ReturnedQuiz {
	pub fn new_from(quiz: Quiz, questions: Vec<Question>, answers: Vec<Answer>, owner: (i32, String)) -> Self {
		let mut collected_questions = questions
			.into_iter().map(|map_question| {
				ReturnedQuestion {
					answers: answers.clone().into_iter().filter(|answer| {
						answer.question_id.eq(&map_question.id)
					}).collect(),
					id: map_question.id,
					quiz_id: map_question.quiz_id,
					question: map_question.question,
					created_at: map_question.created_at,
					updated_at: map_question.updated_at,
					question_rank: map_question.question_rank,
				}
			})
			.collect::<Vec<ReturnedQuestion>>();

		collected_questions.sort_by(|a, b| { a.question_rank.cmp(&b.question_rank) });

		Self {
			id: quiz.id,
			owner: ReturnedUser {
				id: owner.0,
				username: owner.1,
			},
			name: quiz.name,
			created_at: quiz.created_at,
			updated_at: quiz.updated_at,
			questions: collected_questions,
		}
	}
}

async fn get_quiz_by_id (quiz_id: i32, conn: &mut PgConnection) -> Result<ReturnedQuiz, diesel::result::Error> {
	let quiz = quiz::table.find(quiz_id).first::<Quiz>(conn)?;
    let questions = Question::belonging_to(&quiz).load::<Question>(conn)?;
	let answers = Answer::belonging_to(&questions).load::<Answer>(conn)?;
	let owner = users::table
		.filter(users::id.eq(quiz.owner_id))
		.select((users::id, users::username))
		.get_result::<(i32, String)>(conn)?;

	Ok(ReturnedQuiz::new_from(quiz, questions, answers, owner))
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