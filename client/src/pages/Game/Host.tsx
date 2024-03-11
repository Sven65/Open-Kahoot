import { useContext, useEffect } from 'preact/hooks'
import { GameContext } from '../../context/GameContext'
import { Button } from '../../components/Form/Button'
import { Timer } from '../../components/Timer'
import { createRef } from 'preact'
import { toast } from 'react-toastify'
import { Highscores } from '../../components/Highscores'
import { useLocation } from 'preact-iso'

export const Host = () => {
	const gameContext = useContext(GameContext)
	const [roomId] = gameContext.roomId
	const [currentQuestion] = gameContext.currentQuestion
	const [scores] = gameContext.scores
	const { sendHideQuestion, sendShowQuestion, sendNextQuestion, sendGetHighscores } = gameContext

	const location = useLocation()

	const timerRef = createRef()

	useEffect(() => {
		if (!currentQuestion) return
		const newTime = new Date()
		newTime.setSeconds(newTime.getSeconds() + currentQuestion.max_time)

		timerRef.current.restart(newTime)
	}, [currentQuestion])

	if (!roomId) {
		location.route('/')
		return <h1>Please wait...</h1>
	}

	const onTimerExpire = () => {
		toast('timer expited')

		sendGetHighscores()
	}

	console.log('the scores are now', scores)

	return (
		<>
			<h1>Host of game {roomId}</h1>

			{(scores && scores.length > 0) && (
				<Highscores scores={scores} />
			)}

			{currentQuestion && (<h1>Question is: {currentQuestion?.question}</h1>)}

			<Timer timerRef={timerRef} time={currentQuestion?.max_time} onExpire={onTimerExpire} />

			<Button color="red" onClick={sendHideQuestion}>Hide question</Button>
			<Button color="green"  onClick={sendShowQuestion}>Show question</Button>
			<Button color="green"  onClick={sendNextQuestion}>Next question</Button>
		</>
	)
}
