import { createContext } from 'preact'
import { StateUpdater, useState } from 'preact/hooks'
import { io } from 'socket.io-client'
import { Player, SocketEvents } from '../types'
import { LocationHook, useLocation } from 'preact-iso'
import { toast } from 'react-toastify'

export type Answer = {
	id: string,
	answer: string,
}

export type Question = {
	id: string,
    question: string,
    answers: Answer[],
    correct_answer_id: string,
	max_time: number
}

type SocketErrorMessage = {
	error: string,
	error_type: SocketEvents
}

export type IGameContext = {
	roomId: [string, StateUpdater<string>],
	currentQuestion: [Question, StateUpdater<Question>],
	showQuestion: boolean,
	timeLeft: [number, StateUpdater<number>],
	timerInterval: [any, StateUpdater<any>],
	scores: [Player[], StateUpdater<Player[]>],
	// eslint-disable-next-line no-unused-vars
	join: (room_id: string, name: string) => void,
	createRoom: () => void,
	// eslint-disable-next-line no-unused-vars
	sendAnswer: (_answer: string) => void,
	sendShowQuestion: () => void,
	sendHideQuestion: () => void,
	sendNextQuestion: () => void,
	sendGetHighscores: () => void,
	
}

const URL = process.env.NODE_ENV === 'production' ? undefined : 'http://localhost:3000'


export const socket = io(URL)

// @ts-ignore  
window.socket = socket


export const GameContext = createContext<IGameContext>(null)


export const GameContextProvider = ({
	children,
}) => {
	const location = useLocation()

	const [ roomId, setRoomId ] = useState('')
	const [ showQuestion, setShowQuestion ] = useState(false)
	const [ currentQuestion, setCurrentQuestion ] = useState<Question | null>(null)
	let   [ timeLeft, setTimeLeft ] = useState(0)
	const [ timerInterval, setTimerInterval ] = useState(null)
	const [ scores, setScores ] = useState([])

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

	socket.on(SocketEvents.Error, (data: SocketErrorMessage) => {
		toast.error(data.error)
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
		setCurrentQuestion(question)
	})

	socket.on(SocketEvents.GetScores, (scores: Player[]) => {
		setScores(scores)
		console.log('Got scores', scores)
	})

	console.log('qcurrent q in game ctx', currentQuestion)

	return (
		<GameContext.Provider value={{
			join: (room_id: string, name: string) => {
				console.log('Senfing the join', room_id, name)
				socket.emit(SocketEvents.Join, {
					room_id,
					name,
				})
			},
			roomId: [ roomId, setRoomId ],
			currentQuestion: [ currentQuestion, setCurrentQuestion ],
			timeLeft: [ timeLeft, setTimeLeft ],
			timerInterval: [ timerInterval, setTimerInterval ],
			scores: [ scores, setScores ],
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
			sendGetHighscores: () => {
				socket.emit(SocketEvents.GetScores, roomId)
			},
			showQuestion,
		}}>
			{children}
		</GameContext.Provider>
	)
}
