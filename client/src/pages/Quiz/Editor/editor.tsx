import { useRoute } from 'preact-iso'
import { useContext, useEffect, useState } from 'preact/hooks'
import { ApiContext } from '../../../context/ApiContext'

import './editor.scss'
import { DndContext } from '@dnd-kit/core'
import { QuestionEditor } from './QuestionsEditor'
import { Button } from '../../../components/Form/Button'
import { Question, Quiz } from '../../../types'

export const QuizEditor = () => {
	const apiContext = useContext(ApiContext)
	const quiz = apiContext.quiz
	const route = useRoute()

	const { getQuiz, saveQuiz } = apiContext

	const [ editedQuiz, setEditedQuiz ] = useState<Quiz>(null)

	useEffect(() => {
		getQuiz(route.params.id)
	}, [])

	useEffect(() => {
		setEditedQuiz(quiz)
	}, [quiz])

	if (!quiz) return <h1>Please wait...</h1>

	
	console.log('quiz', apiContext.quiz)

	return (
		<div>
			<h1>Editing quiz {quiz.name}</h1>
			<QuestionEditor
				questions={quiz.questions}
				onEdit={(newQuestions: Question[]) => {
					if (!editedQuiz) return
					editedQuiz.questions = newQuestions
					setEditedQuiz(editedQuiz)
				}}
			/>
			<Button onClick={() => saveQuiz(editedQuiz)}>Save</Button>
		</div>
	)
}
