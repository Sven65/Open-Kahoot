import { useContext, useEffect, useRef, useState } from 'preact/hooks'
import { DashboardLayout } from '../../../components/Layouts/Dashboard/Dashboard'
import { ApiContext } from '../../../context/ApiContext'
import { useRoute } from 'preact-iso'
import { Answer, AnswerColor, Question, Quiz, RecursivePartial } from '../../../types'
import { QuestionsList } from './QuestionsList'
import { Button } from '../../../components/Form/Button'
import { Input } from '../../../components/Form/Input'
import { deleteByKey, replaceObjectById } from '../../../util/modify'
import { Card } from '../../../components/Card/Card'
import { AnswerInput } from '../../../components/Form/AnswerInput'
import { DangerModal } from '../../../components/Modal/DangerModal'
import { MobileEditorSidebar } from './MobileEditorSidebar'

export const QuizEditor = () => {
	const apiContext = useContext(ApiContext)
	const quiz = apiContext.quiz
	const route = useRoute()

	const { getQuiz, saveQuiz, deleteQuiz, deleteQuestion } = apiContext

	const [ editedQuiz, setEditedQuiz ] = useState<Quiz>(null)
	const [ selectedQuestion, setSelectedQuestion ] = useState<Question>(null)
	const [ showModal, setShowModal ] = useState(false)

	const [ imagePreview, setImagePreview ] = useState('')

	const fileUploadRef = useRef()

	const [ shownPanel, setShownPanel ] = useState<string>('questions')



	const listRef = useRef()

	useEffect(() => {
		// @ts-ignore
		getQuiz(route.params.id)
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [])

	useEffect(() => {
		setEditedQuiz(quiz)
	}, [quiz])


	if (!editedQuiz) return (
		<DashboardLayout>
			<h1>Please wait...</h1>
		</DashboardLayout>
	)

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

	const changeCorrectAnswer = (e) => {
		const newSelectedQuestion: Question = {
			...selectedQuestion,
			answers: selectedQuestion.answers.map(answer => ({
				...answer,
				is_correct: (e.target.name === answer.answer_color && e.target.checked),
			})),
		}

		setSelectedQuestion(newSelectedQuestion)

		replaceObjectById(editedQuiz.questions,	selectedQuestion.id, newSelectedQuestion)

		setEditedQuiz(editedQuiz)
	}

	const deleteSingleQuestion = (id: string) => {
		if (!id.startsWith('new')) deleteQuestion(id)

		const newQuestions = deleteByKey([...quiz.questions], 'id', id)

		setSelectedQuestion(null)

		setEditedQuiz({
			...editedQuiz,
			questions: newQuestions,
		})

		setShownPanel('questions')
	}

	const onChangeFile = async (e) => {
		const url = URL.createObjectURL(e.target.files[0])

		const id = selectedQuestion.image_id ? selectedQuestion.image_id : await apiContext.getTempId()

		const newSelectedQuestion = {
			...selectedQuestion,
			image: url,
			image_id: id,
		}

		setSelectedQuestion(newSelectedQuestion)

		replaceObjectById(editedQuiz.questions,	selectedQuestion.id, newSelectedQuestion)

		setEditedQuiz(editedQuiz)

		apiContext.uploadFile(id, e.target.files[0])
	}

	return (
		<DashboardLayout
			navbarClass="hidden sm:block"
			navbar={(<MobileEditorSidebar onClick={setShownPanel} />)}
		>
			<DangerModal
				show={showModal}
				onClose={() => setShowModal(false)}
				title={'Are you sure?'}
				text={'Are you sure you want to delete this quiz? This action can not be undone.'}
				onAction={() => deleteQuiz(editedQuiz.id)}
				actionText='Delete'
			/>

			<div class="grid grid-rows-[max-content_1fr] grid-flow-col gap-4">
				<div class="">
					<nav id="relative flex flex-wrap items-center">
						<div class="rounded-xl border border-gray-200 bg-white mt-2 mx-6 py-2 px-2 shadow-md shadow-gray-100 max-h-full h-full flex">
							<div class="flex items-center grow">
								<h1>Editing quiz: </h1>
								<input
									type="text"
									class="ml-2 block p-2.5 text-sm text-gray-900 rounded-e-lg rounded-s-gray-100 rounded border border-gray-300 focus:ring-blue-500 focus:border-blue-500"
									required
									value={editedQuiz.name}
								/>
							</div>
							<div class="float-right flex">
								<Button bgColor="green-500" onClick={() => saveQuiz(editedQuiz)}>Save</Button>
								<Button bgColor="red-500" onClick={() => setShowModal(true)}>Delete</Button>
							</div>
						</div>
					</nav>
				</div>
				<div class={'max-h-full flex flex-col mt-0 mt-[-10px]'}>
					<div class="w-full px-6 py-6 mx-auto drop-zone">
						<div class="flex flex-wrap -mx-3 mb-5 max-h-screen h-full">
							<div class={`w-full max-w-full px-3 mb-6 lg:w-2/12 sm:flex-none xl:mb-0 max-h-screen h-[90vh] grow-0 ${shownPanel === 'questions' ? '' : 'hidden'} sm:block`}>
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
										setShownPanel('editor')
									}}
									onAddQuestion={createNewQuestion}
								/>
							</div>
							{!selectedQuestion ? (
								<>
									<div class={`w-full max-w-full px-3 mb-6 lg:w-10/12 xl:mb-0 flex flex-row min-h-screen justify-center items-center ${shownPanel === 'editor' ? '' : 'hidden'} sm:flex`}>
										Please select a question
									</div>
								</>
							) : (
								<>
									<div class={`w-full max-w-full px-3 mb-6 lg:w-8/12 sm:flex-none xl:mb-0 ${shownPanel === 'editor' ? '' : 'hidden'} sm:block`}>
										<Card title="Editor">
											<div>
												<label for="maxPoints" class="block text-sm font-medium leading-6 text-gray-900">Question</label>
												<div class="mt-2">
													<Input
														required
														placeholder={'Question'}
														value={selectedQuestion.question}
														// @ts-ignore
														onInput={e => setSelectedQuestionValue(e.target.value, 'question')}
														type="text"
													/>
												</div>
											</div>
											<div class="mt-4">
												<AnswerInput
													changeCorrectAnswer={changeCorrectAnswer}
													color={AnswerColor.Red}
													setSelectedQuestionAnswer={setSelectedQuestionAnswer}
													answer={getAnswerForColor(AnswerColor.Red)}
												/>
												<AnswerInput
													changeCorrectAnswer={changeCorrectAnswer}
													color={AnswerColor.Green}
													setSelectedQuestionAnswer={setSelectedQuestionAnswer}
													answer={getAnswerForColor(AnswerColor.Green)}
												/>
												<AnswerInput
													changeCorrectAnswer={changeCorrectAnswer}
													color={AnswerColor.Blue}
													setSelectedQuestionAnswer={setSelectedQuestionAnswer}
													answer={getAnswerForColor(AnswerColor.Blue)}
												/>
												<AnswerInput
													changeCorrectAnswer={changeCorrectAnswer}
													color={AnswerColor.Yellow}
													setSelectedQuestionAnswer={setSelectedQuestionAnswer}
													answer={getAnswerForColor(AnswerColor.Yellow)}
												/>
											</div>
											<div class="mt-4">
												<div class="border border-dashed border-gray-500 relative">
													<input type="file" class="cursor-pointer relative block opacity-0 w-full h-full p-20 z-50" onChange={onChangeFile} ref={fileUploadRef} />
													<div class="text-center p-10 absolute top-0 right-0 left-0 m-auto">
														<h4>
															Drop files anywhere to upload
															<br />or
															<p class="">Select Files</p>
														</h4>
													</div>
												</div>
												<div id="preview" class="my-4 flex">
													{selectedQuestion.image  ? (
														<div>
															<img src={selectedQuestion.image} alt="Preview" />
														</div>
													) : null}
												</div>
											</div>
										</Card>
									</div>
									<div class={`w-full max-w-full px-3 mb-6 lg:w-2/12 sm:flex-none xl:mb-0 ${shownPanel === 'meta' ? '' : 'hidden'} sm:block`}>
										<Card title="Question data">
											<div>
												<label for="maxPoints" class="block text-sm font-medium leading-6 text-gray-900">Max Points</label>
												<div class="mt-2">
													<Input
														required
														placeholder={'1000'}
														value={selectedQuestion.max_points}
														// Todo: Prevent letters in here
														// @ts-ignore
														onChange={(e) => setSelectedQuestionValue(parseInt(e.target.value, 10), 'max_points')}
														type="number"
													/>
												</div>
											</div>
											<div>
												<label for="answerTime" class="block text-sm font-medium leading-6 text-gray-900">Answer Time</label>
												<div class="mt-2">
													<Input
														required
														placeholder={'30'}
														value={selectedQuestion.max_time}
														// Todo: Prevent letters in here
														// @ts-ignore
														onChange={(e) => setSelectedQuestionValue(parseInt(e.target.value, 10), 'max_time')}
														type="number"
													/>
												</div>
											</div>
											<div>
												<div class="mt-2">
													<Button full bgColor="red-500" onClick={() => deleteSingleQuestion(selectedQuestion.id)}>Delete Question</Button>
												</div>
											</div>
										</Card>
									</div>
								</>
							)}
						</div>
					</div>
				</div>
			</div> 
		</DashboardLayout>
	)
}
