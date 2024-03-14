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
	const [scoreMap] = gameContext.scoreMap
	const [gameState] = gameContext.gameState
	const [playerNames] = gameContext.playerNames
	

	console.log('gameContext', gameContext)

	const location = useLocation()

	const { sendNextQuestion, sendGetHighscores, sendStartGame } = gameContext


	const timerRef = createRef()

	useEffect(() => {
		if (!currentQuestion) return
		if (!timerRef.current) return
		
		const newTime = new Date()
		newTime.setSeconds(newTime.getSeconds() + currentQuestion.max_time)

		timerRef.current.restart(newTime)
	}, [currentQuestion])

	useEffect(() => {
		if (!timerRef.current) return
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

	const QuestionPart = () => {
		if (currentQuestion === null) return null
		if (scores && scores.length > 0) return null


		const getAnswerForColor = (color: AnswerColor): Answer => {
			return currentQuestion.answers.find(answer => answer.answer_color === color)
		}

		let redAnswer = getAnswerForColor(AnswerColor.Red)
		let blueAnswer = getAnswerForColor(AnswerColor.Blue)
		let greenAnswer = getAnswerForColor(AnswerColor.Green)
		let yellowAnswer = getAnswerForColor(AnswerColor.Yellow)

		
	
		return (
			<div>
				{currentQuestion && (<h1>Question is: {currentQuestion?.question}</h1>)}

				{timerRef && (<Timer timerRef={timerRef} time={currentQuestion?.max_time} onExpire={onTimerExpire} />)}

				<div className={'answer-shower'}>
					<div class="row">
						<span class="answer-red">{redAnswer && redAnswer.answer}</span>
						<span class="answer-green" >{greenAnswer && greenAnswer.answer}</span>
					</div>
					<div class="row">
						<span class="answer-blue">{blueAnswer && blueAnswer.answer}</span>
						<span class="answer-yellow">{yellowAnswer && yellowAnswer.answer}</span>
					</div>
				</div>
			</div>
		)

	}
		
	const GameScreen = () => {
		const getCorrectAnswerColor = (): AnswerColor => {
			return currentQuestion.answers.find(answer => answer.is_correct).answer_color
		}
	

		return (
			<>
				{(scores && scores.length > 0) && (
					<>
						<Button color="green"  onClick={sendNextQuestion}>Next</Button>
						<Highscores scores={scores} scoreMap={scoreMap} correctAnswerColor={getCorrectAnswerColor()} />
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
