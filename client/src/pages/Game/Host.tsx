import { useContext, useEffect } from 'preact/hooks'
import { GameContext } from '../../context/GameContext'
import { Button } from '../../components/Form/Button'
import { Timer } from '../../components/Timer'
import { createRef } from 'preact'
import { toast } from 'react-toastify'
import { Highscores } from '../../components/Highscores'
import { useLocation } from 'preact-iso'
import { Answer, AnswerColor, GameState } from '../../types'
import { StartingScreen } from '../../components/Host/StartingScreen'

import './Host.scss'

export const Host = () => {
	const gameContext = useContext(GameContext)
	const [roomId] = gameContext.roomId
	const [currentQuestion] = gameContext.currentQuestion
	const [scores] = gameContext.scores
	const [gameState] = gameContext.gameState
	const [playerNames] = gameContext.playerNames
	

	const location = useLocation()

	const { sendNextQuestion, sendGetHighscores, sendStartGame } = gameContext


	const timerRef = createRef()

	useEffect(() => {
		if (!currentQuestion) return
		
		const newTime = new Date()
		newTime.setSeconds(newTime.getSeconds() + currentQuestion.max_time)

		timerRef.current.restart(newTime)
	}, [currentQuestion])

	useEffect(() => {
		console.log('timer effect')
		if (scores && scores.length > 0) {
			console.log('stop timer')
			timerRef.current.pause()
			return
		}
	}, [scores])

	if (!roomId) {
		location.route('/')
		return <h1>Please wait...</h1>
	}

	const onTimerExpire = () => {
		toast('timer expited')

		sendGetHighscores()
	}

	console.log('the scores are now', scores)
	console.log('gamestate', gameState)

	console.log('gamestate is expexte', gameState === GameState.STARTING)

	const QuestionPart = () => {
		if (scores && scores.length > 0) return null


		const getAnswerForColor = (color: AnswerColor): Answer => {
			return currentQuestion.answers.find(answer => answer.answer_color === color)
		}
	

		let redAnswer = getAnswerForColor(AnswerColor.Red)
		let blueAnswer = getAnswerForColor(AnswerColor.Blue)
		let greenAnswer = getAnswerForColor(AnswerColor.Green)
		let yellowAnswer = getAnswerForColor(AnswerColor.Yellow)
	
		return (
			<>
				{currentQuestion && (<h1>Question is: {currentQuestion?.question}</h1>)}

				<Timer timerRef={timerRef} time={currentQuestion?.max_time} onExpire={onTimerExpire} />

				<div className={'answer-shower'}>
					<div class="row">
						<span class="answer-red">{redAnswer.answer}</span>
						<span class="answer-green" >{greenAnswer.answer}</span>
					</div>
					<div class="row">
						<span class="answer-blue">{blueAnswer.answer}</span>
						<span class="answer-yellow">{yellowAnswer.answer}</span>
					</div>
				</div>
			</>
		)

	}
		
	const GameScreen = () => {
		return (
			<>
				{(scores && scores.length > 0) && (
					<>
						<Button color="green"  onClick={sendNextQuestion}>Next</Button>
						<Highscores scores={scores} />
					</>
				)}

				<QuestionPart />
			</>
		)
	}

	return (
		<>
			{gameState === GameState.STARTING && (
				<>
					<StartingScreen
						roomId={roomId}
						names={playerNames}
						onStartGame={sendStartGame}
					/>
				</>
			)} 

			{gameState === GameState.PLAYING && <GameScreen />}

			{gameState === GameState.ENDED && <Highscores scores={scores} />}
		</>
	)
}
