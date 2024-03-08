import { useContext } from 'preact/hooks'
import { GameContext } from '../../context/GameContext'

import './Player.scss'

export const Player = () => {
	const gameContext = useContext(GameContext)
	const [roomId] = gameContext.roomId
	const sendAnswer = gameContext.sendAnswer

	return (
		<>
			<h1>Player of game {roomId}</h1>

			<div className={'play-button-container'}>
				<div class="row">
					<button class="play-btn-red" onClick={() => sendAnswer('1')}>1</button>
					<button class="play-btn-green" onClick={() => sendAnswer('2')}>2</button>
				</div>
				<div class="row">
					<button class="play-btn-blue" onClick={() => sendAnswer('3')}>3</button>
					<button class="play-btn-yellow" onClick={() => sendAnswer('4')}>4</button>
				</div>
			</div>
		</>
	)
}
