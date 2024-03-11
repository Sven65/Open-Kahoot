import { Button } from '../Form/Button'
import './StartingScreen.scss'

interface Props {
	roomId: string,
	names: string[],
	onStartGame: () => void,
}

export const StartingScreen = ({
	roomId,
	names,
	onStartGame,
}: Props) => {	
	return (
		<div class={'starting-screen'}>
			<div class="starting-screen header-container">
				<div class="starting-screen header">
					<div class="starting-screen header pin-text">
						<span>Game PIN:</span>
						<h1>{roomId}</h1>
					</div>
				</div>
			</div>
			<div class="starting-screen names-container">
				<div class="starting-screen names-content">
					{names.map(player => (
						<span class="starting-screen-name" key={player}>{player}</span>
					))}
				</div>
			</div>
			<div class="game-start-button">
				<Button color="green" onClick={onStartGame}>Start game</Button>
			</div>
		</div>
	)
}
