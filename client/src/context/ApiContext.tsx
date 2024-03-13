import { createContext } from 'preact'
import { Quiz } from '../types'
import { useState } from 'preact/hooks'

export type IApiContext = {
	quiz: Quiz,
	getQuiz: (id: number) => Promise<void>,
}

export const ApiContext = createContext<IApiContext>(null)


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
		}}>
			{children}
		</ApiContext.Provider>
	)
}
