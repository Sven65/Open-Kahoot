export enum SocketEvents {
	Error = 'Error',
	Join = 'Join',
	CreateRoom = 'CreateRoom',
	RoomCreated = 'RoomCreated',
	RoomJoined = 'RoomJoined',
	JoinFailed = 'JoinFailed',
	SendAnswer = 'SendAnswer',
	ShowQuestion = 'ShowQuestion',
	HideQuestion = 'HideQuestion',
    SendQuestion = 'SendQuestion',
    NextQuestion = 'NextQuestion',
	GetScores = 'GetScores',
	RoomClosed = 'RoomClosed',
	PlayerLeft = 'PlayerLeft',
	PlayerJoined = 'PlayerJoined',
	ChangeState = 'ChangeState',
}

export type Player = {
	id: string,
	points: number,
	name: string
}

export enum GameState {
	UNKNOWN = 'UNKNOWN',
	STARTING = 'STARTING',
	PLAYING = 'PLAYING',
	ENDED = 'ENDED',
}
