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

export enum AnswerColor {
    Red = 'Red',
    Yellow = 'Yellow',
    Blue = 'Blue',
    Green = 'Green',
}

export type Answer = {
    id: number,
    question_id: number,
    answer: String,
    is_correct: boolean,
    answer_color: AnswerColor,
    created_at: Date,
    updated_at: Date,
}

export type Question = {
	id: number,
	quiz_id: number,
	question: String,
	correct_answer_id: number,
	answers: Answer[],
	question_rank: number,
	max_time: number,
    max_points: number,
	created_at: Date,
	updated_at: Date,
}

export type User = {
	id: number,
	username: String,
}

export type Quiz = {
	id: number,
	owner: User,
	name: String,
	public: boolean,
	created_at: Date,
	updated_at: Date,
	questions: Question[],
}
