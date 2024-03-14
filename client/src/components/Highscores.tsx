import classNames from 'classnames'
import { ScoreMap } from '../context/GameContext'
import { AnswerColor, Player } from '../types'
import { CenterText } from './CenterText'
import './Highscores.scss'

interface Props {
	scores: Player[],
	scoreMap: ScoreMap,
	correctAnswerColor: AnswerColor,
}

export const Highscores = ({
	scores,
	scoreMap,
	correctAnswerColor,
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
			<div class="answer-breakdown">
				{
					Object.entries(scoreMap).map(([ color, count ]) => (
						<div
							class={classNames('answer-count', {
								[color]: true,
								muted: color !== correctAnswerColor,
							})} key={`answer-${color}`}>
							<div class="answer-count-inner">
								<span>{count}</span>
							</div>
						</div>
					))
				}
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
								<span class="highscore-row points">{player.points.toFixed(0)}</span>
							</div>
						</li>
					))}
				</ul>
			</div>
		</div>
	)
}
