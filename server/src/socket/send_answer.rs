use socketioxide::extract::{Data, SocketRef};
use tracing::info;

use super::socket_type::{SocketErrorMessage, SocketEventType};
use crate::{player::calculate_points, GAMEROOM_STORE};

#[derive(Debug, serde::Deserialize, Clone)]
pub struct SentInAnswer {
    pub room_id: String,
    pub answer: String
}


#[derive(Debug, serde::Serialize)]
struct PointsOutMessage {
    pub points: f32,
    pub time_taken: f32,
}


// Todo: Refactor this shit code
pub async fn send_answer(
	socket: SocketRef,
	data: Data<SentInAnswer>,
) {
	let data = data.0;
	// TODO: Check if player is in a room
	let player_answer = data.answer;
	let room_id = data.room_id;


	if let Some(mut room) = GAMEROOM_STORE.get_room_clone(&room_id).await {
		if !room.has_player(socket.id.to_string()) {
			let _ = socket.emit(SocketEventType::Error, "Naughty! You're not in this room.");
			return
		}

		let cloned_room = room.clone();
		let question_started = room.state.question_started.unwrap(); // Clone the field
		let question = cloned_room.get_current_question().unwrap();
		if question.correct_answer_id.is_none() {
			let _ = socket.emit(SocketEventType::Error, SocketErrorMessage {error: "Question does not have a correct answer.".to_string(), error_type: SocketEventType::SendAnswer });
			return
		}

		if player_answer == question.correct_answer_id.clone().unwrap() {
			let question_clone = question.clone();
			if let Some(player) = room.get_player_mut(socket.id.to_string()) {
				if player.has_answered {
					info!("Player has already answered");
					return
				}

				let duration = question_started.elapsed(); // Use the cloned field
				let points = calculate_points(duration.as_secs_f32(), question_clone.max_time, question.max_points);
				player.add_points(points);
				player.has_answered = true;
				player.answer_id = Some(player_answer.clone());


				// Insert the modified room back into the store
				GAMEROOM_STORE.insert(room.clone()).await;

				let _ = socket.emit(SocketEventType::SendPoints, PointsOutMessage {
					points: points,
					time_taken: duration.as_secs_f32(),
				});

				if room.has_all_players_answered() {
					room.reset_answers();
					GAMEROOM_STORE.insert(room.clone()).await;

					let scores = room.get_players_sorted_by_score();
					let answer_counts = room.count_answer_colors();
					let _ = socket.to(room.id).emit(SocketEventType::GetScores, (scores, answer_counts));
				}
			} else {
				let _ = socket.emit(SocketEventType::SendPoints, 0);
			}
		} else {
			if let Some(player) = room.get_player_mut(socket.id.to_string()) {
				if player.has_answered {
					info!("Player has already answered");
					return
				}

				player.has_answered = true;
				player.answer_id = Some(player_answer.clone());

				GAMEROOM_STORE.insert(room.clone()).await;


				if room.has_all_players_answered() {
					room.reset_answers();
					GAMEROOM_STORE.insert(room.clone()).await;

					let scores = room.get_players_sorted_by_score();
					let answer_counts = room.count_answer_colors();
					let _ = socket.to(room.id).emit(SocketEventType::GetScores, (scores, answer_counts));
				}
			}

			

			info!("Sent in answer doesn't match");
		}
	}
}