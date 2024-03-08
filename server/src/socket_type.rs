use core::fmt;
use std::borrow::Cow;


#[derive(serde::Serialize)]
pub enum SocketEventType {
	Join,
	CreateRoom,
	RoomCreated,
	RoomJoined,
	JoinFailed,
}

impl From<SocketEventType> for Cow<'static, str> {
    fn from(event_type: SocketEventType) -> Self {
        match event_type {
            SocketEventType::Join => Cow::Borrowed("Join"),
            SocketEventType::CreateRoom => Cow::Borrowed("CreateRoom"),
            SocketEventType::RoomCreated => Cow::Borrowed("RoomCreated"),
            SocketEventType::RoomJoined => Cow::Borrowed("RoomJoined"),
            SocketEventType::JoinFailed => Cow::Borrowed("JoinFailed"),
        }
    }
}

impl SocketEventType {
    fn to_cow_string(&self) -> Cow<'static, str> {
        match self {
            SocketEventType::Join => Cow::Borrowed("Join"),
            SocketEventType::CreateRoom => Cow::Borrowed("CreateRoom"),
			SocketEventType::RoomCreated => Cow::Borrowed("RoomCreated"),
            SocketEventType::RoomJoined => Cow::Borrowed("RoomJoined"),
            SocketEventType::JoinFailed => Cow::Borrowed("JoinFailed"),
        }
    }
}

#[derive(serde::Serialize)]
pub struct SocketErrorMessage {
    pub error_type: SocketEventType,
    pub error: String
}

