import { useContext, useEffect, useState } from 'preact/hooks'
import { GameContext } from '../../context/GameContext'

import './Player.scss'
import { useLocation } from 'preact-iso'
import { CenterText } from '../../components/CenterText'
import { Answer, AnswerColor } from '../../types'

interface GamePlayerProps {
	onSelect: () => void,
}

const GamePlayer = ({
	onSelect,
}: GamePlayerProps) => {
	const gameContext = useContext(GameContext)
	const sendAnswer = gameContext.sendAnswer
	const [question] = gameContext.currentQuestion

	const getAnswerForColor = (color: AnswerColor): Answer => {
		return question.answers.find(answer => answer.answer_color === color)
	}

	const selectAnswer = (answer: string) => {
		onSelect()
		sendAnswer(answer)
	}

	let redAnswer = getAnswerForColor(AnswerColor.Red)
	let blueAnswer = getAnswerForColor(AnswerColor.Blue)
	let greenAnswer = getAnswerForColor(AnswerColor.Green)
	let yellowAnswer = getAnswerForColor(AnswerColor.Yellow)

	return (
		<div className={'play-button-container'}>
			<div class="row">
				<button class="play-btn-red" onClick={() => selectAnswer(redAnswer.id)}>{redAnswer && redAnswer.answer}</button>
				<button class="play-btn-green" onClick={() => selectAnswer(greenAnswer.id)}>{greenAnswer && greenAnswer.answer}</button>
			</div>
			<div class="row">
				<button class="play-btn-blue" onClick={() => selectAnswer(blueAnswer.id)}>{blueAnswer && blueAnswer.answer}</button>
				<button class="play-btn-yellow" onClick={() => selectAnswer(yellowAnswer.id)}>{yellowAnswer && yellowAnswer.answer}</button>
			</div>
		</div>
	)
}

const PlayerWaiter = () => (
	<div class="player-waiter">
		<CenterText>
			<h1>Please wait for your host to advance.</h1>
		</CenterText>
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
