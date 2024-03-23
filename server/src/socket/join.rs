use socketioxide::extract::{Data, SocketRef};
use tracing::info;

use super::socket_type::{SocketErrorMessage, SocketEventType};

use crate::{player::Player, GAMEROOM_STORE, PLAYER_NAME_LENGTH_LIMIT};

#[derive(Debug, serde::Deserialize)]
pub struct JoinMessage {
    pub room_id: String,
    pub name: String,
}


pub async fn join (
	socket: SocketRef,
	data: Data::<JoinMessage>,
) {
	let data = data.0;
	let room = GAMEROOM_STORE.get_room_clone(&data.room_id).await;


	if room.is_none() {
		info!("Failed to join room {:#?} as it does not exist. {:#?}", data.room_id, socket.rooms());
		let _ = socket.emit(SocketEventType::Error, SocketErrorMessage {
			error_type: SocketEventType::JoinFailed,
			error: "Failed to join as room doesn't exist.".to_string(),
		});
		return;
	}

	if data.name.len() > PLAYER_NAME_LENGTH_LIMIT {
		let _ = socket.emit(SocketEventType::Error, SocketErrorMessage {
			error_type: SocketEventType::JoinFailed,
			error: format!("Failed to join as player name is above limit. ({:#?})", PLAYER_NAME_LENGTH_LIMIT).to_string(),
		});
		return
	}

	let room = &mut room.unwrap();

	let player = Player {
		id: socket.id.to_string(),
		points: 0.0,
		name: Some(data.name),
		has_answered: false,
		answer_id: None,
	};
	
	room.insert_player(player.clone());

	GAMEROOM_STORE.insert(room.clone()).await;

	let _ = socket.leave_all();
	let _ = socket.join(data.room_id.clone());
	
	let _ = socket.emit(SocketEventType::RoomJoined, data.room_id.clone());
	let _ = socket.to(data.room_id.clone()).emit(SocketEventType::PlayerJoined, player);
}