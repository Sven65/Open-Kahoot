import { useCallback, useContext, useEffect, useState } from 'preact/hooks'
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
import { Layout } from '../../components/Layouts/Layout'
import { getImageUrl } from '../../context/ApiContext'

export const Host = () => {
	const gameContext = useContext(GameContext)
	const [roomId] = gameContext.roomId
	const [currentQuestion] = gameContext.currentQuestion
	const [scores] = gameContext.scores
	const [scoreMap] = gameContext.scoreMap
	const [gameState] = gameContext.gameState
	const [playerNames] = gameContext.playerNames

	

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
			timerRef.current.pause()
			return
		}
	}, [scores])

	if (!roomId) {
		location.route('/')
		return <h1>Please wait...</h1>
	}

	const onTimerExpire = () => {
		sendGetHighscores()
	}

	const QuestionPart = () => {
		const [ imageUrl, setImageUrl ] = useState('')
		const fetchImageUrl = useCallback(async () => {
			let url = await getImageUrl(currentQuestion.image_id)

			setImageUrl(url.startsWith('http') ? url : `${window.__env__.REACT_APP_BACKEND_URL}/api${url}`)
		  }, [])

		useEffect(() => {
			fetchImageUrl()
		}, [fetchImageUrl])

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
			<div class="grid grid-rows-3 h-full">
				<div class="bg-white flex justify-center items-center m-24 h-[30%]">
					<div class="float-left flex grow ml-8">
						{timerRef && (<Timer timerRef={timerRef} time={currentQuestion?.max_time} onExpire={onTimerExpire} />)}
					</div>
					<div class="float-middle flex grow">
						<h1 class="text-4xl">{currentQuestion?.question}</h1>
					</div>
					<div class="float-right mr-8">
						Game PIN - {roomId}
					</div>
				</div>

				<div class="flex justify-center">
					<img src={imageUrl} class="h-full max-h-full max-w-full" />
				</div>



				<div className={'answer-shower grid grid-rows-2'}>
					<div class="grid grid-cols-2">
						<span class="answer-red flex justify-center items-center grow">{redAnswer && redAnswer.answer}</span>
						<span class="answer-green flex justify-center items-center grow">{greenAnswer && greenAnswer.answer}</span>
					</div>
					<div class="grid grid-cols-2">
						<span class="answer-blue flex justify-center items-center grow">{blueAnswer && blueAnswer.answer}</span>
						<span class="answer-yellow flex justify-center items-center grow">{yellowAnswer && yellowAnswer.answer}</span>
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
			<div class="bg-slate-600 text-gray-900 max-h-full h-full">
				{(scores && scores.length > 0) && (
					<div>
						<Highscores
							onNext={sendNextQuestion}
							scores={scores}
							scoreMap={scoreMap}
							correctAnswerColor={getCorrectAnswerColor()}
						/>
					</div>
				)}

				<QuestionPart />
			</div>
		)
	}

	return (
		<>
			{gameState === GameState.STARTING && (
				<Layout className="flex justify-center bg-slate-600">
					<StartingScreen
						roomId={roomId}
						names={playerNames}
						onStartGame={sendStartGame}
					/>
				</Layout>
			)} 

			{gameState === GameState.PLAYING && <GameScreen />}

			{gameState === GameState.ENDED && <Highscores scores={scores} scoreMap={scoreMap} correctAnswerColor={null} onNext={() => null} />}
		</>
	)
}
