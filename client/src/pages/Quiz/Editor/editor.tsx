import { useLocation, useRoute } from 'preact-iso'
import { useContext, useEffect, useState } from 'preact/hooks'
import { ApiContext } from '../../../context/ApiContext'

import './editor.scss'
import { QuestionEditor } from './QuestionsEditor'
import { Button } from '../../../components/Form/Button'
import { Question, Quiz } from '../../../types'
import { SortableItem } from './SortableItem'
import { QuestionsList } from './QuestionsList'

export const QuizEditor = () => {
	const apiContext = useContext(ApiContext)
	const quiz = apiContext.quiz
	const location = useLocation()
	const route = useRoute()

	const { getQuiz, saveQuiz } = apiContext

	const [ editedQuiz, setEditedQuiz ] = useState<Quiz>(null)
	const [ selectedQuestionId, setSelectedQuestionId ] = useState<string>(null)
	const [ selectedQuestion, setSelectedQuestion ] = useState<Question>(null)


	useEffect(() => {
		getQuiz(route.params.id)
	}, [])

	useEffect(() => {
		setEditedQuiz(quiz)
	}, [quiz])

	
	if (!editedQuiz) return <h1>Please wait...</h1>
	
	const replaceObjectById = (array: any[], idToReplace: string, newObject: Record<any, any>) => {
		const index = array.findIndex(obj => obj.id === idToReplace)
		if (index !== -1) {
			array[index] = { ...newObject, id: idToReplace } // Preserving the original ID
		}
	}

	// const onEditQuestion = (newQuestion: Question) => {
	// 	const editClone = Object.assign({}, editedQuiz)
	// 	replaceObjectById(editClone.questions, selectedQuestion.id, newQuestion)

	// 	setEditedQuiz(editClone)
	// }
	
	console.log('quiz', apiContext.quiz)

	console.log('ssllc', selectedQuestion)

	return (
		<div class="editor-container">
			<div class="editor-header">
				<div class="editor-header-left">
					<h1>Editing quiz {quiz.name}</h1>
				</div>
				<div class="editor-header-right">
					<Button color="green" onClick={() => saveQuiz(editedQuiz)}>Save</Button>
				</div>
			</div>

			<div class="editor-left-column">
				<QuestionsList
					questions={editedQuiz.questions}
					onEdit={(newQuestions: Question[]) => {
						if (!editedQuiz) return
						editedQuiz.questions = newQuestions
						setEditedQuiz(editedQuiz)
					}}
					onClickQuestion={(id) => {
						setSelectedQuestion(quiz.questions.find(question => question.id === id))
					}}
				/>
			</div>

			<div class="editor-middle-column">
				<QuestionEditor
					question={selectedQuestion}
					onEdit={() => null}
				/>
			</div>

			<div class="editor-right-column">
				right
			</div>
	

			
		</div>
	)
}
