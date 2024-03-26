import classNames from 'classnames'
import { Answer, AnswerColor } from '../../types'

type Props = {
	answer?: Answer,
	color: AnswerColor,
	setSelectedQuestionAnswer: (value: string, color: AnswerColor) => void,
	changeCorrectAnswer: (e: any) => void,
}

export const AnswerInput = ({
	answer,
	color,
	setSelectedQuestionAnswer,
	changeCorrectAnswer,
}: Props) => {
	return (
		<div class="flex">
			<span
				class={classNames('flex-shrink-0 z-10 inline-flex items-center py-2.5 px-4 text-sm font-medium text-center text-gray-900 border border-e-0 border-gray-300 rounded-s-lgfocus:ring-4 focus:outline-none ', [
					`bg-${color.toLowerCase()}-400`,
				])}
			>
				{color.toString()}
			</span>
			<div class="relative w-full">
				<input
					type="text"
					class="block p-2.5 w-full z-20 text-sm text-gray-900 rounded-e-lg rounded-s-gray-100 rounded-s-2 border border-gray-300 focus:ring-blue-500 focus:border-blue-500"
					placeholder={`${color} answer`}
					value={answer?.answer}
					onChange={e => setSelectedQuestionAnswer(e.target.value, color)}
					required
				/>
				<span class="absolute top-0 end-0 p-2.5 h-full text-sm font-medium text-white bg-blue-700 rounded-e-lg focus:outline-none outline-none focus:ring-0 focus:ring-transparent focus:ring-offset-0">
					<input
						type="checkbox"
						class="outline-none focus:ring-0 ring-0 focus:ring-transparent focus:ring-offset-0"
						onChange={changeCorrectAnswer}
						name={color}
						checked={answer?.is_correct}
					/>
				</span>
			</div>
		</div>
	)
}
