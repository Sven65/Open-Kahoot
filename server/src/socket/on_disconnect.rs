use socketioxide::{extract::SocketRef, socket::DisconnectReason};
use tracing::info;

use super::socket_type::SocketEventType;

use crate::GAMEROOM_STORE;

pub async fn on_disconnect(socket: SocketRef, reason: DisconnectReason) {
	info!("Disconnected socket was in rooms {:#?}, because {:#?}", socket.rooms(), reason);
	let player_id = socket.id.to_string();

	if let Some(mut room) = GAMEROOM_STORE.get_player_rooms_cloned(player_id.clone()).await {
		let cloned_room = room.clone();

		if room.host == player_id {

			GAMEROOM_STORE.remove(cloned_room).await;
			
			let _ = socket.to(room.id.clone()).emit(SocketEventType::RoomClosed, "Host left");

			return;
		}

		let player = cloned_room.get_player(player_id.clone());
		room.remove_player(player_id.clone());

		let _ = socket.to(room.id.clone()).emit(SocketEventType::PlayerLeft, player.unwrap());

		GAMEROOM_STORE.insert(room).await;
	} else {
		info!("No room found for disconnected socket.");
	}
}