import { useRoute } from 'preact-iso'
import { useContext, useEffect } from 'preact/hooks'
import { ApiContext } from '../../../context/ApiContext'

import './editor.scss'
import { DndContext } from '@dnd-kit/core'
import { QuestionEditor } from './QuestionsEditor'

export const QuizEditor = () => {
	const apiContext = useContext(ApiContext)
	const quiz = apiContext.quiz
	const route = useRoute()

	useEffect(() => {
		apiContext.getQuiz(route.params.id)
	}, [])

	if (!quiz) return <h1>Please wait...</h1>

	
	console.log('quiz', apiContext.quiz)

	return (
		<div>
			<h1>Editing quiz {quiz.name}</h1>
			<QuestionEditor questions={quiz.questions} />
		</div>
	)
}
