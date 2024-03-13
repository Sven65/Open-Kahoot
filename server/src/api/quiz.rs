use axum::{extract::Path, http::{Response, StatusCode}, routing::get, Json, Router};
use chrono::NaiveDateTime;
use diesel::{prelude::*, QueryDsl};
use tracing::info;

use crate::{api::util::{generic_error, generic_json_response, generic_response, json_response}, db::{establish_connection, models::{Answer, Question, Quiz}, schema::{answers, questions, quiz, users}}};


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReturnedQuestion {
	pub id: i32,
	pub quiz_id: i32,
	pub question: String,
	pub correct_answer_id: Option<i32>,
	pub answers: Vec<Answer>,
	pub question_rank: i32,
	pub max_time: f32,
    pub max_points: f32,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReturnedUser {
	pub id: i32,
	pub username: String,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReturnedQuiz {
	pub id: i32,
	pub owner: ReturnedUser,
	pub name: String,
	pub public: bool,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
	pub questions: Vec<ReturnedQuestion>,
}

impl ReturnedQuiz {
	pub fn new_from(quiz: Quiz, questions: Vec<Question>, answers: Vec<Answer>, owner: (i32, String)) -> Self {
		let mut collected_questions = questions
			.into_iter().map(|map_question| {
				let answers_for_question = answers.clone().into_iter().filter(|answer| {
					answer.question_id.eq(&map_question.id)
				}).collect::<Vec<Answer>>();

				let mut correct_answer_id: Option<i32> = None;

				if let Some(correct_answer) = answers_for_question.iter().find(|answer| {
					answer.is_correct
				}) {
					correct_answer_id = Some(correct_answer.id)
				}


				return ReturnedQuestion {
					answers: answers_for_question.clone(),
					id: map_question.id,
					quiz_id: map_question.quiz_id,
					question: map_question.question,
					created_at: map_question.created_at,
					updated_at: map_question.updated_at,
					question_rank: map_question.question_rank,
					correct_answer_id,
					max_points: map_question.max_points,
					max_time: map_question.max_time,
				};
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
			public: quiz.public,
			created_at: quiz.created_at,
			updated_at: quiz.updated_at,
			questions: collected_questions,
		}
	}
}

pub async fn get_quiz_by_id (quiz_id: i32, conn: &mut PgConnection) -> Result<ReturnedQuiz, diesel::result::Error> {
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

async fn update_quiz(
	Path(_id): Path<i32>,
	Json(new_quiz): Json<ReturnedQuiz>,
) -> Response<axum::body::Body> {
	info!("Payload is {:#?}", new_quiz);

	let mut conn = establish_connection();
	let cloned_quiz = new_quiz.clone();

	for ret_question in new_quiz.questions {
		let update_question = Question::from(ret_question.clone());
		
		let _= diesel::update(crate::api::quiz::questions::dsl::questions)
			.filter(questions::id.eq(update_question.id))
			.set(&update_question)
			.execute(&mut conn);

		for ret_answer in ret_question.answers {			
			let _= diesel::update(crate::api::quiz::answers::dsl::answers)
				.filter(answers::id.eq(ret_answer.id))
				.set(&ret_answer)
				.execute(&mut conn);
		};
	};

	let _= diesel::update(crate::api::quiz::quiz::dsl::quiz)
		.filter(quiz::id.eq(cloned_quiz.id))
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