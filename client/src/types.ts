import type colors from 'tailwindcss/colors'

type FlattenKeys<T extends Record<string, any>, Prefix extends string = ''> = {
	[K in keyof T]-?: T[K] extends Record<string, any>  // @ts-ignore
	  ? FlattenKeys<T[K], `${Prefix}${Prefix extends '' ? '' : '-'}${K}`> // @ts-ignore
	  : `${Prefix}${Prefix extends '' ? '' : '-'}${K}`;
}[keyof T];
  
  
export type TailwindColor = FlattenKeys<typeof colors>;

export type TailwindComponentColors = {
	bgColor?: TailwindColor,
	textColor?: TailwindColor,
	hoverColor?: TailwindColor,
	focusColor?: TailwindColor,
}

export type RecursivePartial<T> = {
	[P in keyof T]?:
	  T[P] extends (infer U)[] ? RecursivePartial<U>[] :
	  T[P] extends object | undefined ? RecursivePartial<T[P]> :
	  T[P];
  };
  

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
    id: string,
    question_id: string,
    answer: string,
    is_correct: boolean,
    answer_color: AnswerColor,
    created_at: Date,
    updated_at: Date,
}

export type Question = {
	id: string,
	quiz_id: string,
	question: string,
	correct_answer_id: string,
	answers: Answer[],
	question_rank: number,
	image_id?: string,
	max_time: number,
    max_points: number,
	created_at: Date,
	updated_at: Date,
}

export type User = {
	id: string,
	username: string,
	avatar?: string,
}

export type Quiz = {
	id: string,
	owner: User,
	name: String,
	public: boolean,
	created_at: Date,
	updated_at: Date,
	questions: Question[],
}
