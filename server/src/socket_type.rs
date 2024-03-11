use std::borrow::Cow;


#[derive(serde::Serialize)]
pub enum SocketEventType {
    Error,
	Join,
	CreateRoom,
	RoomCreated,
	RoomJoined,
	JoinFailed,
    SendAnswer,
    SendPoints,
    ShowQuestion,
	HideQuestion,
    SendQuestion,
    NextQuestion,
    GameOver,
}

impl From<SocketEventType> for Cow<'static, str> {
    fn from(event_type: SocketEventType) -> Self {
        match event_type {
            SocketEventType::Error => Cow::Borrowed("Error"),
            SocketEventType::Join => Cow::Borrowed("Join"),
            SocketEventType::CreateRoom => Cow::Borrowed("CreateRoom"),
            SocketEventType::RoomCreated => Cow::Borrowed("RoomCreated"),
            SocketEventType::RoomJoined => Cow::Borrowed("RoomJoined"),
            SocketEventType::JoinFailed => Cow::Borrowed("JoinFailed"),
            SocketEventType::SendAnswer => Cow::Borrowed("SendAnswer"),
            SocketEventType::SendPoints => Cow::Borrowed("SendPoints"),
            SocketEventType::ShowQuestion => Cow::Borrowed("ShowQuestion"),
            SocketEventType::HideQuestion => Cow::Borrowed("HideQuestion"),
            SocketEventType::SendQuestion => Cow::Borrowed("SendQuestion"),
            SocketEventType::NextQuestion => Cow::Borrowed("NextQuestion"),
            SocketEventType::GameOver => Cow::Borrowed("GameOver"),
        }
    }
}

impl SocketEventType {
    fn to_cow_string(&self) -> Cow<'static, str> {
        match self {
            SocketEventType::Error => Cow::Borrowed("Error"),
            SocketEventType::Join => Cow::Borrowed("Join"),
            SocketEventType::CreateRoom => Cow::Borrowed("CreateRoom"),
			SocketEventType::RoomCreated => Cow::Borrowed("RoomCreated"),
            SocketEventType::RoomJoined => Cow::Borrowed("RoomJoined"),
            SocketEventType::JoinFailed => Cow::Borrowed("JoinFailed"),
            SocketEventType::SendAnswer => Cow::Borrowed("SendAnswer"),
            SocketEventType::SendPoints => Cow::Borrowed("SendPoints"),
            SocketEventType::ShowQuestion => Cow::Borrowed("ShowQuestion"),
            SocketEventType::HideQuestion => Cow::Borrowed("HideQuestion"),
            SocketEventType::SendQuestion => Cow::Borrowed("SendQuestion"),
            SocketEventType::NextQuestion => Cow::Borrowed("NextQuestion"),
            SocketEventType::GameOver => Cow::Borrowed("GameOver"),
        }
    }
}

#[derive(serde::Serialize)]
pub struct SocketErrorMessage {
    pub error_type: SocketEventType,
    pub error: String
}

