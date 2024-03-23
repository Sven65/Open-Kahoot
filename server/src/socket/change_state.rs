use socketioxide::extract::{Data, SocketRef};
use tracing::info;

use crate::{socket::socket_type::SocketErrorMessage, GAMEROOM_STORE};
use super::socket_type::SocketEventType;





#[derive(Debug, serde::Deserialize)]
pub struct ChangeStateMessage {
    pub room_id: String,
    pub state: String,
}

pub async fn change_state(
	socket: SocketRef,
	data: Data::<ChangeStateMessage>,
) {
	let data = data.0;
	if let Some(mut room) = GAMEROOM_STORE.get_room_clone(&data.room_id).await {
		if !room.is_host(socket.id.to_string()) {
			let _ = socket.emit(SocketEventType::Error, "Naughty! This isn't your room.");
			return
		}

		room.set_client_state(data.state.clone());

		GAMEROOM_STORE.insert(room).await;

		let _ = socket.emit(SocketEventType::ChangeState, data.state.clone());
		let _ = socket.to(data.room_id).emit(SocketEventType::ChangeState, data.state);
	} else {
		info!("No room exists");
		let _ = socket.emit(SocketEventType::Error, SocketErrorMessage {error: "Room doesn't exist.".to_string(), error_type: SocketEventType::ChangeState });
	}
}