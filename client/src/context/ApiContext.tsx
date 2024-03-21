import { createContext } from 'preact'
import { Quiz, User } from '../types'
import { useState } from 'preact/hooks'
import { toast } from 'react-toastify'
import { useLocation } from 'preact-iso'

export type CreateUser = {
	username: string,
	password: string,
	email: string,
}


export type IApiContext = {
	quiz: Quiz,
	user: User,
	// eslint-disable-next-line no-unused-vars
	getQuiz: (id: number) => Promise<void>,
	// eslint-disable-next-line no-unused-vars
	saveQuiz: (quiz: Quiz) => Promise<void>,
	// eslint-disable-next-line no-unused-vars
	deleteQuiz: (id: String) => Promise<void>,
	// eslint-disable-next-line no-unused-vars
	deleteQuestion: (id: String) => Promise<void>,
	// eslint-disable-next-line no-unused-vars
	createUser: (user: CreateUser) => Promise<void>,
	// eslint-disable-next-line no-unused-vars
	login: (username: string, password: string) => Promise<void>,
	fetchMe: () => Promise<void>,
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
	const [ user, setUser ] = useState<User>(null)
	const location = useLocation()

	return (
		<ApiContext.Provider value={{
			quiz,
			user,
			getQuiz: async (id: number) => {
				const request = await fetch(`/api/quiz/${id}`)
				const data = await request.json()

				setQuiz(data)
			},
			saveQuiz: async (quiz: Quiz) => {
				const request = await fetch(`/api/quiz/${quiz.id}`, {
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

				const request = await fetch(`/api/question/${id}`, {
					method: 'DELETE',
					headers: {
						'Content-Type': 'application/json',
					},
				})

				if (request.status === 204) {
					toast.success('Delete OK!')
				} else {
					toast.error('Delete failed.')
				}
			},
			// eslint-disable-next-line no-unused-vars
			deleteQuiz: async (id: string) => null,
			createUser: async (user: CreateUser) => {
				const request = await fetch('/§api/user', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
					},
					body: JSON.stringify(user),
				})

				const data = await request.json()

				switch (request.status) {
					case 201:
						toast.success('User created!')
						break
					case 409:
						toast.error(data.error)
						break
					default:
						toast.error('User creation failed.')
				}
			},
			login: async (username: string, password: string) => {
				const request = await fetch('/api/user/login', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
					},
					body: JSON.stringify({
						username,
						password,
					}),
				})

				const data = await request.json()
				console.log('data', data)

				if (request.status === 200) {
					toast.success('Logged in.')

					location.route('/@me')
				} else {
					toast.error(`Login failed: ${data.error}`)
				}
			},
			fetchMe: async () => {
				const request = await fetch('/api/user/@me', {
					method: 'GET',
					headers: {
						'Content-Type': 'application/json',
					},
				})

				const data = await request.json()

				if (request.status === 200) {
					toast.success('Logged in.')

					setUser(data)

					// location.route('/@me')
				} else {
					toast.error(`Login failed: ${data.error}`)
				}
			},
		}}>
			{children}
		</ApiContext.Provider>
	)
}
