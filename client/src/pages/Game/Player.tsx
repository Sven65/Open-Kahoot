import { useContext, useEffect, useState } from 'preact/hooks'
import { GameContext } from '../../context/GameContext'

import './Player.scss'
import { useLocation } from 'preact-iso'

interface GamePlayerProps {
	onSelect: () => void,
}

const GamePlayer = ({
	onSelect,
}: GamePlayerProps) => {
	const gameContext = useContext(GameContext)
	const sendAnswer = gameContext.sendAnswer
	const [question] = gameContext.currentQuestion

	const selectAnswer = (answer: string) => {
		onSelect()
		sendAnswer(answer)
	}


	return (
		<div className={'play-button-container'}>
			<div class="row">
				<button class="play-btn-red" onClick={() => selectAnswer(question.answers[0].id)}>{question.answers[0].answer}</button>
				<button class="play-btn-green" onClick={() => selectAnswer(question.answers[1].id)}>{question.answers[1].answer}</button>
			</div>
			<div class="row">
				<button class="play-btn-blue" onClick={() => selectAnswer(question.answers[2].id)}>{question.answers[2].answer}</button>
				<button class="play-btn-yellow" onClick={() => selectAnswer(question.answers[3].id)}>{question.answers[3].answer}</button>
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
	const location = useLocation()

	const gameContext = useContext(GameContext)
	const [currentQuestion] = gameContext.currentQuestion
	const [roomId] = gameContext.roomId
	const showQuestion = gameContext.showQuestion
	const [ hasSelected, setHasSelected ] = useState(false)

	useEffect(() => {
		if (showQuestion && hasSelected) {
			setHasSelected(false)
		}
	}, [currentQuestion])


	if (!roomId) {
		location.route('/')
		return <h1>Please wait...</h1>
	}


	return (
		<>
			<h1>Player of game {roomId}</h1>

			{/* TODO: Varying messages about waiting for host and points and stuff */}

			{(showQuestion && !hasSelected)  ? <GamePlayer onSelect={() => setHasSelected(true)} /> : <PlayerWaiter />}
		</>
	)
}
