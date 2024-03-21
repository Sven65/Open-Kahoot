import { createContext } from 'preact'
import { StateUpdater, useState } from 'preact/hooks'
import { io } from 'socket.io-client'
import { AnswerColor, GameState, Player, SocketEvents } from '../types'
import { useLocation } from 'preact-iso'
import { toast } from 'react-toastify'

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
	scoreMap: [ScoreMap, StateUpdater<ScoreMap>],
	playerNames: [string[], StateUpdater<string[]>],
	gameState: [GameState, StateUpdater<GameState>],
	// eslint-disable-next-line no-unused-vars
	join: (room_id: string, name: string) => void,
	// eslint-disable-next-line no-unused-vars
	createRoom: (quizId: string) => void,
	// eslint-disable-next-line no-unused-vars
	sendAnswer: (_answer: string) => void,
	sendShowQuestion: () => void,
	sendHideQuestion: () => void,
	sendNextQuestion: () => void,
	sendGetHighscores: () => void,
	sendStartGame: () => void,
	
	clearContext: () => void,
}

// const URL = process.env.NODE_ENV === 'production' ? undefined : 'http://localhost:3000'
// @ts-ignore
const URL = window.__env__.REACT_APP_BACKEND_URL

export const socket = io(URL)

// @ts-ignore  
window.socket = socket


export const GameContext = createContext<IGameContext>(null)
export type ScoreMap = Record<AnswerColor, number>;


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
	const [ scoreMap, setScoreMap ] = useState<ScoreMap | null>(null)
	const [ gameState, setGameState ] = useState<GameState>(GameState.STARTING)

	const [ playerNames, setPlayerNames ] = useState<string[]>([])

	const clearContext = () => {
		setScores([])
		setScoreMap(null)
		setRoomId('')
		setShowQuestion(false)
		setCurrentQuestion(null)
		setTimeLeft(0)
		setTimerInterval(null)
		setPlayerNames([])
		setGameState(GameState.STARTING)
	}

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
		console.log('question has length', question.answers.length)
		
		setCurrentQuestion(question)

		setScores([])
	})

	socket.on(SocketEvents.GetScores, (scores: Player[], counts: ScoreMap) => {
		setScores(scores)
		setScoreMap(counts)
		console.log('Got scores', scores)
	})

	socket.on(SocketEvents.RoomClosed, () => {
		toast.error('Left game as room closed.')
		clearContext()
		location.route('/')
	})

	socket.on(SocketEvents.PlayerJoined, (player: Player) => {
		setPlayerNames([
			player.name,
			...playerNames,
		])
	})

	socket.on(SocketEvents.PlayerLeft, (player: Player) => {
		setPlayerNames(playerNames.filter(item => item !== player.name))
	})

	socket.on(SocketEvents.ChangeState, (state: GameState) => {
		setGameState(state)
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
			scoreMap: [ scoreMap, setScoreMap ],
			playerNames: [ playerNames, setPlayerNames ],
			gameState: [ gameState, setGameState ],
			createRoom: (quizId: string) => {
				socket.emit(SocketEvents.CreateRoom, quizId)
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
			sendStartGame: () => {
				console.log('starting game')
				socket.emit(SocketEvents.ChangeState, { room_id: roomId, state: GameState.PLAYING })
				socket.emit(SocketEvents.NextQuestion, roomId)
			},
			clearContext,
			showQuestion,
		}}>
			{children}
		</GameContext.Provider>
	)
}
