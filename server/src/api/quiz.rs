use axum::{extract::Path, http::{Response, StatusCode}, routing::get, Json, Router};
use chrono::NaiveDateTime;
use diesel::{prelude::*, QueryDsl};
use tracing::info;

use crate::{api::util::{generic_error, generic_json_response, json_response}, db::{establish_connection, models::{Answer, Question, Quiz, RealAnswerColor}, schema::{questions, quiz, users}}};


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReturnedAnswer {
    pub id: String,
    pub question_id: String,
    pub answer: String,
    pub is_correct: bool,
    pub answer_color: RealAnswerColor,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl From<Answer> for ReturnedAnswer {
	fn from(value: Answer) -> Self {
		Self {
			id: value.id,
			question_id: value.question_id,
			answer: value.answer,
			is_correct: value.is_correct,
			answer_color: value.answer_color,
			created_at: Some(value.created_at),
			updated_at: Some(value.updated_at),
		}
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReturnedQuestion {
	pub id: String,
	pub quiz_id: String,
	pub question: String,
	pub correct_answer_id: Option<String>,
	pub answers: Vec<ReturnedAnswer>,
	pub question_rank: i32,
	pub max_time: f32,
    pub max_points: f32,
	pub created_at: Option<NaiveDateTime>,
	pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReturnedUser {
	pub id: String,
	pub username: String,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReturnedQuiz {
	pub id: String,
	pub owner: ReturnedUser,
	pub name: String,
	pub public: bool,
	pub created_at: Option<NaiveDateTime>,
	pub updated_at: Option<NaiveDateTime>,
	pub questions: Vec<ReturnedQuestion>,
}

impl ReturnedQuiz {
	pub fn new_from(quiz: Quiz, questions: Vec<Question>, answers: Vec<Answer>, owner: (String, String)) -> Self {
		let mut collected_questions = questions
			.into_iter().map(|map_question| {
				let answers_for_question = answers.clone().into_iter().filter_map(|answer| {
					if answer.question_id == map_question.id {
						Some(ReturnedAnswer::from(answer))
					} else {
						None
					}
				}).collect::<Vec<ReturnedAnswer>>();

				let mut correct_answer_id: Option<String> = None;

				if let Some(correct_answer) = answers_for_question.iter().find(|answer| {
					answer.is_correct
				}) {
					correct_answer_id = Some(correct_answer.id.clone())
				}


				return ReturnedQuestion {
					answers: answers_for_question.clone(),
					id: map_question.id,
					quiz_id: map_question.quiz_id,
					question: map_question.question,
					created_at: Some(map_question.created_at),
					updated_at: Some(map_question.updated_at),
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
			created_at: Some(quiz.created_at),
			updated_at: Some(quiz.updated_at),
			questions: collected_questions,
		}
	}
}

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

	for ret_question in new_quiz.questions {
		let update_question = Question::from(ret_question.clone());
		
		// let _= diesel::update(crate::api::quiz::questions::dsl::questions)
		// 	.filter(questions::id.eq(update_question.id))
		// 	.set(&update_question)
		// 	.execute(&mut conn);

		let _ = diesel::insert_into(crate::api::quiz::questions::dsl::questions)
			.values(&update_question)
			.on_conflict(questions::id)
			.do_update()
			.set(&update_question)
			.execute(&mut conn);

		// for ret_answer in ret_question.answers {			
		// 	let _= diesel::update(crate::api::quiz::answers::dsl::answers)
		// 		.filter(answers::id.ueq(ret_answer.id))
		// 		.set(&ret_answer)
		// 		.execute(&mut conn);
		// };
	};

	let _= diesel::update(crate::api::quiz::quiz::dsl::quiz)
		.filter(quiz::id.eq(cloned_quiz.id.clone()))
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