import { useContext, useState } from 'preact/hooks'
import { Button } from '../../components/Form/Button'
import { Input } from '../../components/Form/Input'
import './style.scss'
import { GameContext } from '../../context/GameContext'
import { Modal } from '../../components/Modal/Modal'
import { toast } from 'react-toastify'

export function Home() {
	const gameContext = useContext(GameContext)
	const [ gamePin, setGamePin ] = useState('')
	const [ playerName, setPlayerName ] = useState('')
	const [ showModal, setShowModal ] = useState(false)

	console.log(gameContext)


	const sendJoin = () => {
		console.log('Joining game', gamePin, 'with name', playerName)
		gameContext.join(gamePin, playerName)
	}

	return (
		<div class="home">
			<Modal show={showModal} onClose={() => setShowModal(false)}>
				<div class="join-modal-container">
					<h1>Enter name</h1>
					<Input
						name="player-name"
						placeholder={'Player Name'}
						value={playerName}
						onChange={(evt) => setPlayerName(evt.target.value)}
						onEnter={sendJoin}
					/>
					<Button
						color="green"
						onClick={sendJoin}
						type={'button'}
					>
						Enter
					</Button>
				</div>
			</Modal>
			<div class="form-container">
				<h1>Open Kahoot</h1>
				<form class={'code-form'} action="#" onSubmit={(e) => e.preventDefault()}>
					<Input
						name="code"
						placeholder={'Game PIN'}
						type={'number'}
						value={gamePin}
						onChange={e => setGamePin(e.target.value)}
						onEnter={() => {
							if (gamePin.length === 0) {
								toast.error('Please enter a game PIN.')
								return
							}

							setShowModal(true)
						}}
					/>
					<Button
						color="green"
						onClick={() => {
							if (gamePin.length === 0) {
								toast.error('Please enter a game PIN.')
								return
							}

							setShowModal(true)
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

