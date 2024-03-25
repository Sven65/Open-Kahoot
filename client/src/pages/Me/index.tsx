import { useContext, useEffect, useState } from 'preact/hooks'
import { RequireLogin } from '../../components/HoC/RequireLogin'
import { ApiContext } from '../../context/ApiContext'
import { useLocation } from 'preact-iso'
import { Button } from '../../components/Form/Button'
import { GameContext } from '../../context/GameContext'
import { Input } from '../../components/Form/Input'
import { DashboardLayout } from '../../components/Layouts/Dashboard/Dashboard'
import { InputModal } from '../../components/Modal/InputModal'

const InternalMe = () => {
	const apiContext = useContext(ApiContext)
	const gameContext = useContext(GameContext)
	const location = useLocation()

	useEffect(() => {
		if (!apiContext.userQuizzes) {
			apiContext.fetchUserQuizzes()
		}
	}, [])

	const [ quizName, setQuizName ] = useState('')
	const [ showModal, setShowModal ] = useState(false)


	if (!apiContext.user) return <h1>Please wait, logging in.</h1>
	if (!apiContext.userQuizzes) return <h1>Please wait, loading quizzes.</h1>

	return (
		<DashboardLayout>
			<InputModal
				show={showModal}
				onClose={() => setShowModal(false)}
				title='Enter quiz name'
				actionText='Create!'
				onAction={apiContext.createQuiz}
			/>

			<h1>Welcome, {apiContext.user.username}</h1>

			<h1>Quizzes:</h1>
			{apiContext.userQuizzes.map((quiz) => (
				<div key={quiz.id}>
					{quiz.name}
					<Button onClick={() => location.route(`/quiz/${quiz.id}/edit`)} bgColor="green-500">Edit</Button>
					<Button onClick={() => { gameContext.createRoom(quiz.id) }} bgColor="green-500">Host</Button>
				</div>
			))}
			<Button bgColor="green-500" onClick={() => setShowModal(true)}>Create new quiz</Button>
		</DashboardLayout>
	)
}

export const Me = RequireLogin(InternalMe)
