import { useContext, useState } from 'preact/hooks'
import { Button } from '../../components/Form/Button'
import { Input } from '../../components/Form/Input'
import './style.scss'
import { GameContext } from '../../context/GameContext'

export function Home() {
	const gameContext = useContext(GameContext)
	const [roomId] = gameContext.roomId
	const [ gamePin, setGamePin ] = useState('')

	console.log(gameContext)

	return (
		<div class="home">
			<div class="form-container">
				<h1>Open Kahoot</h1>
				<h1>hello: {roomId}</h1>
				<form class={'code-form'} action="">
					<Input
						name="code"
						placeholder={'Game PIN'}
						value={gamePin}
						onChange={e => setGamePin(e.target.value)}
					/>
					<Button
						color="green"
						onClick={() => {
							gameContext.join(gamePin)
						}}
						type={'button'}
					>
						Enter
					</Button>
					<Button color="" type={'button'} onClick={gameContext.createRoom}>Host</Button>
				</form>
			</div>
		</div>
	)
}

