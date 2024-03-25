import { useContext, useEffect, useRef, useState } from 'preact/hooks'
import { DashboardLayout } from '../../../components/Layouts/Dashboard/Dashboard'
import { ApiContext } from '../../../context/ApiContext'
import { useRoute } from 'preact-iso'
import { AnswerColor, Question, Quiz, RecursivePartial } from '../../../types'
import { QuestionsList } from './QuestionsList'
import { Button } from '../../../components/Form/Button'

export const QuizEditor2 = () => {
	const apiContext = useContext(ApiContext)
	const quiz = apiContext.quiz
	const route = useRoute()

	const { getQuiz, saveQuiz, deleteQuiz, deleteQuestion } = apiContext

	const [ editedQuiz, setEditedQuiz ] = useState<Quiz>(null)
	const [ selectedQuestion, setSelectedQuestion ] = useState<Question>(null)


	const listRef = useRef()

	useEffect(() => {
		// @ts-ignore
		getQuiz(route.params.id)
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [])

	useEffect(() => {
		setEditedQuiz(quiz)
	}, [quiz])

	const createNewQuestion = () => {
		const newQuestion: RecursivePartial<Question> = {
			answers: [
				{ answer: '', is_correct: false, answer_color: AnswerColor.Red },
				{ answer: '', is_correct: false, answer_color: AnswerColor.Green },
				{ answer: '', is_correct: false, answer_color: AnswerColor.Yellow },
				{ answer: '', is_correct: false, answer_color: AnswerColor.Blue },
			],
			correct_answer_id: '',
			quiz_id: editedQuiz.id,
			question: 'New Question',
			question_rank: editedQuiz.questions.length,
			id: `new-${(Math.random() * 16).toString(16)}`,
			max_points: 1000,
			max_time: 30,
		}


		setSelectedQuestion(newQuestion)

		const newQuestions: Question[] = [
			...editedQuiz.questions,
			newQuestion,
		]
		
		setEditedQuiz({
			...editedQuiz,
			questions: newQuestions,
		})
	}


	if (!editedQuiz) return (
		<DashboardLayout>
			<h1>Please wait...</h1>
		</DashboardLayout>
	)

	return (
		<DashboardLayout>
			<nav id="relative flex flex-wrap items-center">
				<h1>Hello navbar</h1>
				<Button color="green" onClick={() => saveQuiz(editedQuiz)}>Save</Button>
			</nav>
			<div class="w-full px-6 py-6 mx-auto drop-zone">
				<div class="flex flex-wrap -mx-3 mb-5">
					<div class="w-full max-w-full px-3 mb-6 lg:w-2/12 sm:flex-none xl:mb-0">
						<QuestionsList
							ref={listRef}
							questions={editedQuiz.questions}
							onEdit={(newQuestions: Question[]) => {
								if (!editedQuiz) return
								editedQuiz.questions = newQuestions
								setEditedQuiz(editedQuiz)
							}}
							onClickQuestion={(id) => {
								setSelectedQuestion(editedQuiz.questions.find(question => question.id === id))
							}}
							onAddQuestion={createNewQuestion}
						/>
					</div>
					<div class="w-full max-w-full px-3 mb-6 lg:w-8/12 sm:flex-none xl:mb-0">col 2</div>
					<div class="w-full max-w-full px-3 mb-6 lg:w-2/12 sm:flex-none xl:mb-0">col 3</div>
				</div>
			</div>
		</DashboardLayout>
	)
}
