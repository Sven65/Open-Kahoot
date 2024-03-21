import { useContext, useEffect } from 'preact/hooks'
import { RequireLogin } from '../../components/HoC/RequireLogin'
import { ApiContext } from '../../context/ApiContext'
import { useLocation } from 'preact-iso'
import { Button } from '../../components/Form/Button'
import { GameContext } from '../../context/GameContext'

const InternalMe = () => {
	const apiContext = useContext(ApiContext)
	const gameContext = useContext(GameContext)
	const location = useLocation()
	console.log('apiContext', apiContext)

	useEffect(() => {
		if (!apiContext.userQuizzes) {
			apiContext.fetchUserQuizzes()
		}
	}, [])


	if (!apiContext.user) return <h1>Please wait, logging in.</h1>
	if (!apiContext.userQuizzes) return <h1>Please wait, loading quizzes.</h1>

	return (
		<div>
			<h1>Welcome, {apiContext.user.username}</h1>

			<h1>Quizzes:</h1>
			{apiContext.userQuizzes.map((quiz) => (
				<div key={quiz.id}>
					{quiz.name}
					<Button onClick={() => location.route(`/quiz/${quiz.id}/edit`)}>Edit</Button>
					<Button onClick={() => { gameContext.createRoom(quiz.id) }} color="green">Host</Button>
				</div>
			))}
			<Button color="green">Create new quiz</Button>
		</div>
	)
}

export const Me = RequireLogin(InternalMe)
