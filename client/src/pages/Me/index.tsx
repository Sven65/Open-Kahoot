import { useContext, useEffect, useState } from 'preact/hooks'
import { RequireLogin } from '../../components/HoC/RequireLogin'
import { ApiContext } from '../../context/ApiContext'
import { useLocation } from 'preact-iso'
import { Button } from '../../components/Form/Button'
import { GameContext } from '../../context/GameContext'
import { DashboardLayout } from '../../components/Layouts/Dashboard/Dashboard'
import { InputModal } from '../../components/Modal/InputModal'
import { Card } from '../../components/Card/Card'
  
const InternalMe = () => {
	const apiContext = useContext(ApiContext)
	const gameContext = useContext(GameContext)
	const location = useLocation()

	useEffect(() => {
		if (!apiContext.userQuizzes) {
			apiContext.fetchUserQuizzes()
		}
	}, [])

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
				placeholder='My awesome quiz'
				icon={(<path strokeLinecap="round" strokeLinejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L6.832 19.82a4.5 4.5 0 0 1-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 0 1 1.13-1.897L16.863 4.487Zm0 0L19.5 7.125" />)}
			/>

			<Card
				title={(
					<div class="flex justify-center">
						<span>Quizzes</span>
						<Button bgColor="green-500" onClick={() => setShowModal(true)} className="ml-8">Create new quiz</Button>
					</div>
				)}
				className={'-px-8 -py-8'}
			>
				<ul role="list" className="divide-y divide-gray-100">
					{apiContext.userQuizzes.map((quiz) => (
						<li key={quiz.id} className="flex justify-between gap-x-6 py-5">
							<div className="flex min-w-0 gap-x-4">
								<div class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-gray-100 sm:mx-0 sm:h-10 sm:w-10">
									<svg class="h-6 w-6 text-gray-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
										<path strokeLinecap="round" strokeLinejoin="round" d="M12 6.042A8.967 8.967 0 0 0 6 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 0 1 6 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 0 1 6-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0 0 18 18a8.967 8.967 0 0 0-6 2.292m0-14.25v14.25" />
									</svg>
								</div>
								<div className="min-w-0 flex-auto">
									<p className="text-sm font-semibold leading-6 text-gray-900">{quiz.name}</p>
									<p className="mt-1 truncate text-xs leading-5 text-gray-500">Placeholder text</p>
								</div>
							</div>
							<div className="hidden shrink-0 sm:flex sm:flex-col sm:items-end">
								<Button onClick={() => location.route(`/quiz/${quiz.id}/edit`)} bgColor="green-500" className="w-full">Edit</Button>
								<Button onClick={() => { gameContext.createRoom(quiz.id) }} bgColor="green-500" className="w-full">Host</Button>
							</div>
						</li>
					))}
				</ul>
			</Card>
		</DashboardLayout>
	)
}

export const Me = RequireLogin(InternalMe)
