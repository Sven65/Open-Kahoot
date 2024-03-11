import { useContext } from 'preact/hooks'
import { GameContext } from '../../context/GameContext'

import './Player.scss'

const GamePlayer = () => {
	const gameContext = useContext(GameContext)
	const sendAnswer = gameContext.sendAnswer
	const [question] = gameContext.currentQuestion


	return (
		<div className={'play-button-container'}>
			<div class="row">
				<button class="play-btn-red" onClick={() => sendAnswer(question.answers[0].id)}>{question.answers[0].answer}</button>
				<button class="play-btn-green" onClick={() => sendAnswer(question.answers[1].id)}>{question.answers[1].answer}</button>
			</div>
			<div class="row">
				<button class="play-btn-blue" onClick={() => sendAnswer(question.answers[2].id)}>{question.answers[2].answer}</button>
				<button class="play-btn-yellow" onClick={() => sendAnswer(question.answers[3].id)}>{question.answers[3].answer}</button>
			</div>
		</div>
	)
}

const PlayerWaiter = () => (
	<div class="player-waiter">
		<h1>Please wait for your host to advance.</h1>
	</div>
)

export const Player = () => {
	const gameContext = useContext(GameContext)
	const [roomId] = gameContext.roomId
	const showQuestion = gameContext.showQuestion

	return (
		<>
			<h1>Player of game {roomId}</h1>

			{showQuestion ? <GamePlayer /> : <PlayerWaiter />}
		</>
	)
}
