import { createContext } from 'preact'
import { Quiz } from '../types'
import { useState } from 'preact/hooks'
import { toast } from 'react-toastify'

export type IApiContext = {
	quiz: Quiz,
	getQuiz: (id: number) => Promise<void>,
	saveQuiz: (quiz: Quiz) => Promise<void>,
	deleteQuiz: (id: String) => Promise<void>,
	deleteQuestion: (id: String) => Promise<void>,
}

export const ApiContext = createContext<IApiContext>(null)
 
const removeNewQuestionIds = (quiz: Quiz): Quiz => {
	return {
		...quiz,
		questions: quiz.questions.map(question => ({
			...question,
			id: question.id.startsWith('new-') ? '' : question.id,
		})),
	}
}

export const ApiContextProvider = ({
	children,
}) => {
	const [ quiz, setQuiz ] = useState<Quiz>(null)

	return (
		<ApiContext.Provider value={{
			quiz,
			getQuiz: async (id: number) => {
				const request = await fetch(`${window.__env__.REACT_APP_BACKEND_URL}/api/quiz/${id}`)
				const data = await request.json()

				setQuiz(data)
			},
			saveQuiz: async (quiz: Quiz) => {
				const request = await fetch(`${window.__env__.REACT_APP_BACKEND_URL}/api/quiz/${quiz.id}`, {
					method: 'PUT',
					headers: {
						'Content-Type': 'application/json',
					},
					body: JSON.stringify(removeNewQuestionIds(quiz)),
				})

				if (request.status === 200) {
					toast.success('Save OK!')
				} else {
					toast.error('Save failed.')
				}
			},
			deleteQuestion: async (id: string) => {
				if (id.startsWith('new-')) return

				const request = await fetch(`${window.__env__.REACT_APP_BACKEND_URL}/api/question/${id}`, {
					method: 'DELETE',
					headers: {
						'Content-Type': 'application/json',
					},
				})

				if (request.status === 201) {
					toast.success('Delete OK!')
				} else {
					toast.error('Delete failed.')
				}
			},
			deleteQuiz: async (id: string) => null,
		}}>
			{children}
		</ApiContext.Provider>
	)
}
