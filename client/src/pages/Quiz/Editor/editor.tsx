import { useRoute } from 'preact-iso'
import { useContext, useEffect } from 'preact/hooks'
import { ApiContext } from '../../../context/ApiContext'


export const QuizEditor = () => {
	const apiContext = useContext(ApiContext)
	const route = useRoute()

	useEffect(() => {
		apiContext.getQuiz(route.params.id)

	}, [])

	console.log('quiz', apiContext.quiz)

	return (
		<div>
			<h1>Hello World</h1>
		</div>
	)
}
