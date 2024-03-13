use chrono::NaiveDateTime;

use crate::db::models::{Answer, Question, Quiz, RealAnswerColor};


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReturnedAnswer {
    #[serde(default)]
	pub id: Option<String>,
    #[serde(default)]
	pub question_id: Option<String>,
    pub answer: String,
    pub is_correct: bool,
    pub answer_color: RealAnswerColor,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl From<Answer> for ReturnedAnswer {
	fn from(value: Answer) -> Self {
		Self {
			id: Some(value.id),
			question_id: Some(value.question_id),
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
	#[serde(default)]
	pub id: Option<String>,
	pub quiz_id: String,
	pub question: String,
	pub correct_answer_id: Option<String>,
	#[serde(default)]
	pub answers: Option<Vec<ReturnedAnswer>>,
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
	pub id: Option<String>,
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
					correct_answer_id = correct_answer.id.clone()
				}


				return ReturnedQuestion {
					answers: Some(answers_for_question.clone()),
					id: Some(map_question.id),
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
			id: Some(quiz.id),
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
