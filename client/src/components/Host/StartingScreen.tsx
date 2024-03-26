import { arrayRandom } from '../../util/array'
import { Button } from '../Form/Button'
import './StartingScreen.scss'

interface Props {
	roomId: string,
	names: string[],
	onStartGame: () => void,
}

const newNames = [
	'Alice',
	'Bob',
	'Charlie',
	'Diana',
	'Eleanor',
	'Frank',
	'Grace',
	'Hannah',
	'Ian',
	'Jasmine',
	'Kevin',
	'Linda',
	'Michael',
	'Nancy',
	'Oliver',
	'Pamela',
	'Quincy',
	'Rachel',
	'Samuel',
	'Tina',
	'Uma',
	'Victor',
	'Wendy',
	'Xavier',
	'Yvonne',
	'Zachary',
]



const getColor = (): string => {
	return arrayRandom([
		'text-cyan-500',
		'text-teal-500',
		'text-red-500',
		'text-green-500',
		'text-orange-500',
		'text-yellow-500',
	])
}

const getSize = (): string => {
	return arrayRandom([
		'text-3xl',
		'text-xl',
		'text-md',
		'text-lg',
		'text-sm',
		'text-4xl',
	  ])
}

export const StartingScreen = ({
	roomId,
	names,
	onStartGame,
}: Props) => {	
	const namers = [ ...names, ...newNames ]
	
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
			<div class="starting-screen names-container mt-8">
				<ul class="flex justify-center flex-wrap max-w-xl align-center gap-2 leading-8">
					{namers.map(player => (
						<li class={`${getColor()} ${getSize()}`} key={player}>{player}</li>
					))}
				</ul>
			</div>
			<div class="game-start-button">
				<Button onClick={onStartGame}>Start game</Button>
			</div>
		</div>
	)
}
