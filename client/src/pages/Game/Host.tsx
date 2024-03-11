import { useContext, useEffect, useState } from 'preact/hooks'
import { GameContext } from '../../context/GameContext'
import { Button } from '../../components/Form/Button'
import { Timer } from '../../components/Timer'

export const Host = () => {
	const gameContext = useContext(GameContext)
	const [roomId] = gameContext.roomId
	const [currentQuestion] = gameContext.currentQuestion
	const { sendHideQuestion, sendShowQuestion, sendNextQuestion } = gameContext
	const [timeLeft] = gameContext.timeLeft

	return (
		<>
			<h1>Host of game {roomId}</h1>
			{currentQuestion && (<h1>Question is: {currentQuestion?.question}</h1>)}

			{currentQuestion && (<Timer time={currentQuestion.max_time} />)}

			<div className="timer">
				{timeLeft && (
					<div>
						seconds: {timeLeft}
					</div>
				)}
			</div>

			<Button color="red" onClick={sendHideQuestion}>Hide question</Button>
			<Button color="green"  onClick={sendShowQuestion}>Show question</Button>
			<Button color="green"  onClick={sendNextQuestion}>Next question</Button>
		</>
	)
}
