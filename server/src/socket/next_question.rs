use std::time::Instant;

use socketioxide::extract::{Data, SocketRef};
use super::socket_type::{SocketErrorMessage, SocketEventType};
use crate::GAMEROOM_STORE;

pub async fn next_question(
	socket: SocketRef,
	room_id: Data::<String>
) {
	let room_id = room_id.0;

	let room = GAMEROOM_STORE.get_room_clone(&room_id).await;

	if let Some(mut room) = room {
		if !room.is_host(socket.id.to_string()) {
			let _ = socket.emit(SocketEventType::Error, "Naughty! This isn't your room.");
			return
		}

		room.prepare_next_question();

		let question = room.get_current_question();

		let _ = socket.emit(SocketEventType::SendQuestion, question.clone());
		let _ = socket.to(room_id.clone()).emit(SocketEventType::SendQuestion, question);
		let _ = socket.to(room_id.clone()).emit(SocketEventType::ShowQuestion, "");
		room.state.question_started = Some(Instant::now());

		GAMEROOM_STORE.insert(room.clone()).await;

		if room.state.is_game_over {
			let _ = socket.emit(SocketEventType::ChangeState, "ENDED");
			let _ = socket.to(room_id).emit(SocketEventType::ChangeState, "ENDED");

			let scores = room.get_players_sorted_by_score();
			let _ = socket.emit(SocketEventType::GetScores, (scores,));
		}
	} else {
		let _ = socket.emit(SocketEventType::Error, SocketErrorMessage {error: "Room doesn't exist.".to_string(), error_type: SocketEventType::NextQuestion });
	}
}