import classNames from 'classnames'
import { ScoreMap } from '../context/GameContext'
import { AnswerColor, Player } from '../types'
import { CenterText } from './CenterText'
import './Highscores.scss'
import { Button } from './Form/Button'

interface Props {
	scores: Player[],
	scoreMap: ScoreMap,
	correctAnswerColor: AnswerColor,
	onNext: () => void,
}

const getDefaultScoreMap = (): ScoreMap => ({
	Blue: 0,
	Green: 0,
	Red: 0,
	Yellow: 0,
})

export const Highscores = ({
	scores,
	scoreMap,
	correctAnswerColor,
	onNext,
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
			<div>
				<div class="answer-breakdown">
					{
						Object.entries(scoreMap || getDefaultScoreMap()).map(([ color, count ]) => (
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
				<div class="flex justify-center">
					<div class="w-[5.5%]">
						<Button full onClick={onNext} bgColor='green-500' hoverColor='green-800'>
							Next
						</Button>
					</div>
				</div>
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
