import { Player } from '../types'

interface Props {
	scores: Player[],
}

export const Highscores = ({
	scores,
}: Props) => {
	return (
		<ul class="highscore-container">
			{scores.map((player, idx) => (
				<li class="highscore-row" key={`score-${idx}`}>
					<span class="highscore-row position">{idx + 1}</span>
					<span class="highscore separator"> - </span>
					<span class="highscore-row name">{player.name}</span>
					<span class="highscore separator"> - </span>
					<span class="highscore-row points">{player.points}</span>
				</li>
			))}
		</ul>
		
	)
}
