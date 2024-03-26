import { useContext, useState } from 'preact/hooks'
import { Button } from '../../components/Form/Button'
import { Input } from '../../components/Form/Input'
import './style.scss'
import { GameContext } from '../../context/GameContext'
import { toast } from 'react-toastify'
import { InputModal } from '../../components/Modal/InputModal'
import { Layout } from '../../components/Layouts/Layout'

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
		<Layout className="home bg-zinc-400 flex justify-center">
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
				<h1 class="text-4xl">Open Kahoot</h1>
				<form class={'code-form mt-8'} action="#" onSubmit={(e) => e.preventDefault()}>
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
		</Layout>
	)
}

