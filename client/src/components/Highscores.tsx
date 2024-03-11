import { Player } from '../types'
import { CenterText } from './CenterText'
import './Highscores.scss'

interface Props {
	scores: Player[],
}

export const Highscores = ({
	scores,
}: Props) => {
	return (
		<div class="highscore-page">
			<div class="header-container">
				<CenterText className="scoreboard-text">
					<div class="inner">
						<h1>Scoreboard</h1>
					</div>
				</CenterText>
			</div>

			<div class="highscore-container">
				<ul class="highscore-list">
					{scores.map((player, idx) => (
						<li class="highscore-row" key={`score-${idx}`}>
							<div class="highscore-row-inner">
								<span class="highscore-row position">{idx + 1}</span>
								<span class="highscore separator"> - </span>
								<span class="highscore-row name">{player.name}</span>
								<span class="highscore separator"> - </span>
								<span class="highscore-row points">{player.points}</span>
							</div>
						</li>
					))}
				</ul>
			</div>
		</div>
	)
}
