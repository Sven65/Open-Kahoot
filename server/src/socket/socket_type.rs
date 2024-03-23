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
    GetScores,
    RoomClosed,
    PlayerLeft,
    PlayerJoined,
    ChangeState,
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
            SocketEventType::GetScores => Cow::Borrowed("GetScores"),
            SocketEventType::RoomClosed => Cow::Borrowed("RoomClosed"),
            SocketEventType::PlayerLeft => Cow::Borrowed("PlayerLeft"),
            SocketEventType::PlayerJoined => Cow::Borrowed("PlayerJoined"),
            SocketEventType::ChangeState => Cow::Borrowed("ChangeState"),
        }
    }
}


#[derive(serde::Serialize)]
pub struct SocketErrorMessage {
    pub error_type: SocketEventType,
    pub error: String
}

