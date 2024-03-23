use socketioxide::extract::{Data, SocketRef};

use crate::{socket_type::{SocketErrorMessage, SocketEventType}, GAMEROOM_STORE};

pub async fn get_scores(
	socket: SocketRef,
	room_id: Data::<String>,
) {
	let room_id = room_id.0;
	if let Some(room) = GAMEROOM_STORE.get_room_clone(&room_id).await {
		let scores = room.get_players_sorted_by_score();
		let _ = socket.emit(SocketEventType::GetScores, (scores,));
	} else {
		let _ = socket.emit(SocketEventType::Error, SocketErrorMessage {error: "Room doesn't exist.".to_string(), error_type: SocketEventType::GetScores });
	}
}