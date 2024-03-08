import { useContext } from 'preact/hooks'
import { GameContext } from '../../context/GameContext'

export const Host = () => {
	const gameContext = useContext(GameContext)
	const [roomId] = gameContext.roomId

	return (
		<h1>Host of game {roomId}</h1>
	)
}
