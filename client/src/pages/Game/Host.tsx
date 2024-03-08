import { useContext } from 'preact/hooks'
import { GameContext } from '../../context/GameContext'
import { Button } from '../../components/Form/Button'

export const Host = () => {
	const gameContext = useContext(GameContext)
	const [roomId] = gameContext.roomId
	const { sendHideQuestion, sendShowQuestion } = gameContext

	return (
		<>
			<h1>Host of game {roomId}</h1>

			<Button color="red" onClick={sendHideQuestion}>Hide question</Button>
			<Button color="green"  onClick={sendShowQuestion}>Show question</Button>
		</>
	)
}
