import { useRoute } from 'preact-iso'
import { useContext, useEffect, useRef, useState } from 'preact/hooks'
import { ApiContext } from '../../../context/ApiContext'

import './editor.scss'
import { Button } from '../../../components/Form/Button'
import { Answer, AnswerColor, Question, Quiz, RecursivePartial } from '../../../types'
import { QuestionsList } from './QuestionsList'
import { Input } from '../../../components/Form/Input'
import { deleteByKey, replaceObjectById } from '../../../util/modify'

export const QuizEditor = () => {
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
	
	if (!editedQuiz) return <h1>Please wait...</h1>

	const getAnswerForColor = (color: AnswerColor): Answer | undefined => {
		return selectedQuestion.answers.find(answer => answer.answer_color === color)
	}

	const setSelectedQuestionValue = (value: any, prop: string) => {
		const newSelectedQuestion = {
			...selectedQuestion,
			[prop]: value,
		}

		setSelectedQuestion(newSelectedQuestion)

		replaceObjectById(editedQuiz.questions,	selectedQuestion.id, newSelectedQuestion)

		setEditedQuiz(editedQuiz)
	}

	const setSelectedQuestionAnswer = (value: string, color: AnswerColor) => {
		const colorIndex = selectedQuestion.answers.findIndex(answer => answer.answer_color === color)
		const colorAnswer = selectedQuestion.answers[colorIndex]
		const answerClone = [...selectedQuestion.answers]

		answerClone[colorIndex] = {
			...colorAnswer,
			answer: value,
		}

		const newSelectedQuestion: Question = {
			...selectedQuestion,
			answers: answerClone,
		}

		setSelectedQuestion(newSelectedQuestion)

		replaceObjectById(editedQuiz.questions,	selectedQuestion.id, newSelectedQuestion)

		setEditedQuiz(editedQuiz)
	}

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

		console.log('new', newQuestion)

		setSelectedQuestion(newQuestion)

		const newQuestions: Question[] = [
			...editedQuiz.questions,
			newQuestion,
		]

		console.log('new leng', newQuestions.length)

		setEditedQuiz({
			...editedQuiz,
			questions: newQuestions,
		})
	}

	const deleteSingleQuestion = (id: string) => {
		console.log('deleting id', id)

		if (!id.startsWith('new')) deleteQuestion(id)

		const newQuestions = deleteByKey([...quiz.questions], 'id', id)

		console.log('newQuestions', newQuestions)

		setSelectedQuestion(null)


		setEditedQuiz({
			...editedQuiz,
			questions: newQuestions,
		})

	}

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
				<div class="row">
					<h1>Question List</h1>
				</div>
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
				/>
				<Button color='green' onClick={createNewQuestion}>New Question</Button>
			</div>

			<div class="editor-middle-column">
				{selectedQuestion ? (
					<div class="single-question-editor">
						<div class="row">
							<h1>Answers & Question</h1>
						</div>
						<div class="row">
							<div class="answer-editor">
								<Input
									label="Question"
									labelClass='white-label'
									placeholder={'Question'}
									value={selectedQuestion.question}
									// @ts-ignore
									onInput={e => setSelectedQuestionValue(e.target.value, 'question')}
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
											value={getAnswerForColor(AnswerColor.Red)?.answer}
											// @ts-ignore
											onChange={e => setSelectedQuestionAnswer(e.target.value, AnswerColor.Red)}
										/>
									</div>
									<div class="answer-editor">
										<Input
											placeholder={'Answer'}
											label={'Green'}
											labelClass="green-label"
											value={getAnswerForColor(AnswerColor.Green)?.answer}
											// @ts-ignore
											onChange={e => setSelectedQuestionAnswer(e.target.value, AnswerColor.Green)}
										/>
									</div>
								</div>
								<div class="row">
									<div class="answer-editor">
										<Input
											placeholder={'Answer'}
											label={'Blue'}
											labelClass="blue-label"
											value={getAnswerForColor(AnswerColor.Blue)?.answer}
											// @ts-ignore
											onChange={e => setSelectedQuestionAnswer(e.target.value, AnswerColor.Blue)}
										/>
									</div>
									<div class="answer-editor">
										<Input
											placeholder={'Answer'}
											label={'Yellow'}
											labelClass="yellow-label"
											value={getAnswerForColor(AnswerColor.Yellow)?.answer}
											// @ts-ignore
											onChange={e => setSelectedQuestionAnswer(e.target.value, AnswerColor.Yellow)}
										/>
									</div>
								</div>
							</div>
						</div>
					</div>
				) : (<h1>Please select a question</h1>)}
			</div>

			<div class="editor-right-column">
				{selectedQuestion && (
					<>
						<div class="row">
							<h1>Question Meta</h1>
						</div>
						<div class="row">
							<div class="answer-editor">
								<Input
									label="Max Points"
									placeholder={'1000'}
									labelClass='white-label'
									value={selectedQuestion.max_points}
									// Todo: Prevent letters in here
									// @ts-ignore
									onChange={(e) => setSelectedQuestionValue(parseInt(e.target.value, 10), 'max_points')}

									type="number"
								/>
							</div>
						</div>
						<div class="row">
							<div class="answer-editor">
								<Input
									label="Answer Time"
									labelClass='white-label'
									placeholder={'30'}
									value={selectedQuestion.max_time}
									// @ts-ignore
									onChange={(e) => setSelectedQuestionValue(parseInt(e.target.value, 10), 'max_time')}
									type="number"
								/>
							</div>
						</div>
						<div class="row">
							<Button color="red" onClick={() => deleteSingleQuestion(selectedQuestion.id)}>Delete Question</Button>
						</div>
					</>
				)}
			</div>
	

			
		</div>
	)
}
