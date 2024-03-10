import { createContext } from 'preact'
import { StateUpdater, useState } from 'preact/hooks'
import { io } from 'socket.io-client'
import { SocketEvents } from '../types'
import { LocationHook, useLocation } from 'preact-iso'

export type Answer = {
	id: string,
	answer: string,
}

export type Question = {
	id: string,
    question: string,
    answers: Answer[],
    correct_answer_id: string,
}

export type IGameContext = {
	join: (room: string) => void,
	roomId: [string, StateUpdater<string>],
	currentQuestion: [Question, StateUpdater<Question>],
	createRoom: () => void,
	locationHook: LocationHook,
	sendAnswer: (answer: string) => void,
	sendShowQuestion: () => void,
	sendHideQuestion: () => void,
	sendNextQuestion: () => void,
	showQuestion: boolean,
}

const URL = process.env.NODE_ENV === 'production' ? undefined : 'http://localhost:3000'


export const socket = io(URL)


export const GameContext = createContext<IGameContext>(null)


export const GameContextProvider = ({
	children,
}) => {
	const location = useLocation()


	const [ roomId, setRoomId ] = useState('')
	const [ showQuestion, setShowQuestion ] = useState(false)
	const [ currentQuestion, setCurrentQuestion ] = useState<Question | null>(null)

	socket.on(SocketEvents.RoomCreated, (roomCode: string) => {
		console.log('room_code', roomCode)
		setRoomId(roomCode)

		location.route('/host')
	})

	socket.on(SocketEvents.RoomJoined, (roomCode: string) => {
		console.log('Room joined successfully', roomCode)
		setRoomId(roomCode)

		location.route('/play')
	})


	socket.on(SocketEvents.JoinFailed, (data) => {
		console.log('join fail', data)
	})

	socket.on(SocketEvents.ShowQuestion, () => {
		setShowQuestion(true)
	})

	socket.on(SocketEvents.HideQuestion, () => {
		setShowQuestion(false)
	})

	socket.on(SocketEvents.SendQuestion, (question: Question) => {
		console.log('got q', question)
		setCurrentQuestion(question)
	})

	console.log('qcurrent q in game ctx', currentQuestion)

	return (
		<GameContext.Provider value={{
			join: (room: string) => {
				console.log('joining room', room)
				socket.emit(SocketEvents.Join, room)
			},
			roomId: [ roomId, setRoomId ],
			currentQuestion: [ currentQuestion, setCurrentQuestion ],
			createRoom: () => {
				socket.emit(SocketEvents.CreateRoom)
			},
			sendAnswer: (answer: string) => {
				socket.emit(SocketEvents.SendAnswer, roomId, answer)
			},
			sendShowQuestion: () => {
				socket.emit(SocketEvents.ShowQuestion, roomId)
			},
			sendHideQuestion: () => {
				socket.emit(SocketEvents.HideQuestion, roomId)
			},
			sendNextQuestion: () => {
				socket.emit(SocketEvents.NextQuestion, roomId)
			},
			showQuestion,
		}}>
			{children}
		</GameContext.Provider>
	)
}
