use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use socketioxide::extract::{Data, SocketRef, State};
use tracing::info;

use crate::{api::{quiz::get_quiz_by_id, quiz_types::ReturnedQuestion}, app_state::AppState, game_room::{GameRoom, GameState}, socket_type::{SocketErrorMessage, SocketEventType}, util, FIRST_QUESTION_ID, GAMEROOM_STORE, LAST_QUESTION_ID};

pub async fn create_room(
	socket: SocketRef,
	state: State<Arc<AppState>>,
	quiz_id: Data::<String>,
) {
	info!("Creating room");
	let quiz_id = quiz_id.0;

	let room_code = util::generate_random_number_string(6);

	let _ = socket.leave_all();
	let _ = socket.join(room_code.clone());
	info!("Rooms are now {:#?}", socket.rooms());

	let mut conn = state.db_pool.get().expect("Failed to get database pool");

	let quiz = get_quiz_by_id(quiz_id.to_string(), &mut conn).await;

	if quiz.is_err() {
		let _ = socket.emit(SocketEventType::Error, SocketErrorMessage {error: "Tried to load a quiz that doesn't exist.".to_string(), error_type: SocketEventType::CreateRoom });
		return
	}

	let quiz = quiz.unwrap();

	let mut questions: Vec<ReturnedQuestion> = vec![
		ReturnedQuestion {
			answers: Some(vec![]),
			correct_answer_id: Some("0".to_string()),
			question: "This should never be shown".to_string(),
			id: Some(FIRST_QUESTION_ID.to_string()),
			max_time: 30.0,
			max_points: 1000.0,
			created_at: Some(Utc::now().naive_utc()),
			updated_at: Some(Utc::now().naive_utc()),
			question_rank: 0,
			quiz_id: quiz_id.to_string(),
		}
	];

	let mut questions_with_answers: Vec<ReturnedQuestion> = quiz.questions
		.iter()
		.filter(|question| {
			if let Some(answers) = &question.answers {
				if answers.len() == 0 {
					return false;
				}
				true
			} else {
				false
			}
		})
		.map(|question| {
			question.to_owned()
		})
		.collect::<Vec<ReturnedQuestion>>();

	questions.append(&mut questions_with_answers);

	questions.push(ReturnedQuestion {
		answers: Some(vec![]),
		correct_answer_id: Some("0".to_string()),
		question: "This should never be shown.".to_string(),
		id: Some(LAST_QUESTION_ID.to_string()),
		max_time: 30.0,
		max_points: 1000.0,
		created_at: Some(Utc::now().naive_utc()),
		updated_at: Some(Utc::now().naive_utc()),
		question_rank: std::i32::MAX,
		quiz_id: quiz_id.to_string(),
	});

	GAMEROOM_STORE.insert(GameRoom {
		id: room_code.clone(),
		host: socket.id.to_string(),
		players: HashMap::new(),
		state: GameState {
			show_question: false,
			current_question_id: FIRST_QUESTION_ID.to_string(),
			is_game_over: false,
			question_started: None,
			answer_count: 0,
			client_state: "UNKNOWN".to_string(),
		},
		questions: questions,
	}).await;


	socket.emit(SocketEventType::RoomCreated, room_code).unwrap();
}