import { useEffect, useState } from 'preact/hooks'
import { Answer, AnswerColor, Question } from '../../../types'
import { Input } from '../../../components/Form/Input'

import './QuestionsEditor.scss'

interface Props {
	question: Question,
	onEdit: (newQuestion: Question) => void
}

export const QuestionEditor = ({
	question,
	onEdit,
}: Props) => {
	const [ editedQuestion, setEditedQuestion ] = useState(question)

	useEffect(() => {
		onEdit(editedQuestion)
	}, [editedQuestion])

	useEffect(() => {
		setEditedQuestion(question)
	}, [question])

	const setQuestion = (e) => {
		setEditedQuestion({
			...editedQuestion,
			question: e.target.value,
		})

		// onEdit(editedQuestion)
	}

	const getAnswerForColor = (color: AnswerColor): Answer => {
		return editedQuestion.answers.find(answer => answer.answer_color === color)
	}


	if (!editedQuestion) return <h1>Loading...</h1>


	return (
		<div class="single-question-editor">
			<div class="row">
				<div class="answer-editor">
					<Input
						label="Question"
						labelClass='white-label'
						placeholder={'Question'}
						value={editedQuestion.question}
						onInput={setQuestion}
					/>
				</div>
			</div>
			<div class="row">

				<div class="single-q-answer-editor">
					<div class="row">
						<div class="answer-editor red">
							<Input
								id="red-answer"
								placeholder={'Answer'}
								label={'Red'}
								labelClass="red-label"
								value={getAnswerForColor(AnswerColor.Red).answer}
							/>
						</div>
						<div class="answer-editor">
							<Input
								placeholder={'Answer'}
								label={'Green'}
								labelClass="green-label"
								value={getAnswerForColor(AnswerColor.Green).answer}
							/>
						</div>
					</div>
					<div class="row">
						<div class="answer-editor">
							<Input
								placeholder={'Answer'}
								label={'Blue'}
								labelClass="blue-label"
								value={getAnswerForColor(AnswerColor.Blue).answer}
							/>
						</div>
						<div class="answer-editor">
							<Input
								placeholder={'Answer'}
								value={getAnswerForColor(AnswerColor.Yellow).answer}
								label={'Yellow'}
								labelClass="yellow-label"
							/>
						</div>
					</div>
				</div>
			</div>
		</div>
	)
}
