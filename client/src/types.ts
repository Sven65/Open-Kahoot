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
}

export type Player = {
	id: string,
	points: number,
	name: string
}
