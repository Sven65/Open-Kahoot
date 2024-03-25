import { useContext, useState } from 'preact/hooks'
import { Button } from '../../components/Form/Button'
import { Input } from '../../components/Form/Input'
import './style.scss'
import { GameContext } from '../../context/GameContext'
import { Modal } from '../../components/Modal/Modal'
import { toast } from 'react-toastify'
import { InputModal } from '../../components/Modal/InputModal'


const people = [
	{
	  name: 'Calvin Hawkins',
	  email: 'calvin.hawkins@example.com',
	  image:
		'https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
	},
	{
	  name: 'Kristen Ramos',
	  email: 'kristen.ramos@example.com',
	  image:
		'https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
	},
	{
	  name: 'Ted Fox',
	  email: 'ted.fox@example.com',
	  image:
		'https://images.unsplash.com/photo-1500648767791-00dcc994a43e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
	},
]
  

export function Home() {
	const gameContext = useContext(GameContext)
	const [ gamePin, setGamePin ] = useState('')
	const [ showModal, setShowModal ] = useState(false)

	const sendJoin = (playerName: string) => {
		if (!playerName || playerName.length === 0) {
			toast.error('Please enter a name!')
			return
		} 
		gameContext.join(gamePin, playerName)
	}

	return (
		<div class="home">
			<InputModal
				show={showModal}
				onClose={() => setShowModal(false)}
				onAction={sendJoin}
				title="Enter name"
				actionText='Enter'
				placeholder='Mike'
				icon={(<path strokeLinecap="round" strokeLinejoin="round" d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z" />)}
			/>
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
				</form>
			</div>
		</div>
	)
}

