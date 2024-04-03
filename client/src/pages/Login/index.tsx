import { useContext, useRef, useState } from 'preact/hooks'
import { Button } from '../../components/Form/Button'
import { Input } from '../../components/Form/Input'

import { ApiContext, requestResetPassword } from '../../context/ApiContext'
import { toast } from 'react-toastify'

import './Login.scss'
import { useLocation } from 'preact-iso'
import { InputModal } from '../../components/Modal/InputModal'
import { validateEmail } from '../../util/validator'

export const Login = () => {
	const [ username, setUsername ] = useState(null)
	const [ password, setPassword ] = useState(null)
	const [ isSent, setIsSent ] = useState(false)

	const [ showForgotForm, setShowForgotForm ] = useState(false)

	const formRef = useRef()

	const apiContext = useContext(ApiContext)
	const location = useLocation()


	const submitForm = () => {
		if(!formRef.current.checkValidity()) return toast.error('Form is invalid. Please check your inputs.')

		apiContext.login(
			username,
			password,
		)
	}

	const submitReset = async (value: string) => {
		if (value === '') return toast.error('Please enter a valid email!')
		if (!validateEmail(value)) return toast.error('Please enter a valid email!')

		requestResetPassword(value)

		toast.success('Success! An email has been sent if it exists.')
		setShowForgotForm(false)
	}

	return (
		<div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8 bg-white">
			<InputModal
				show={showForgotForm}
				text='Please enter your email'
				title='Forgot password'
				actionText='Send'
				icon={(
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
						<path strokeLinecap="round" strokeLinejoin="round" d="M16.5 12a4.5 4.5 0 1 1-9 0 4.5 4.5 0 0 1 9 0Zm0 0c0 1.657 1.007 3 2.25 3S21 13.657 21 12a9 9 0 1 0-2.636 6.364M16.5 12V8.25" />
					</svg>
				)}
				onClose={() => setShowForgotForm(false)}
				onAction={submitReset}
			/>
			<div class="sm:mx-auto sm:w-full sm:max-w-sm">
				<img class="mx-auto h-10 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600" alt="Your Company" />
				<h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">Sign in to your account</h2>
			</div>

			<div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
				<form class="space-y-6" ref={formRef} onSubmit={e => e.preventDefault()}>
					<div>
						<label for="email" class="block text-sm font-medium leading-6 text-gray-900">Username</label>
						<div class="mt-2">
							<Input
								id="username"
								name="username"
								type="text"
								autocomplete="username"
								required
								value={username}
								onChange={e => setUsername(e.target.value)}
							/>
						</div>
					</div>

					<div>
						<div class="flex items-center justify-between">
							<label for="password" class="block text-sm font-medium leading-6 text-gray-900">Password</label>
							<div class="text-sm">
								<a href="#" class="font-semibold text-indigo-600 hover:text-indigo-500" onClick={() => setShowForgotForm(true)}>Forgot password?</a>
							</div>
						</div>
						<div class="mt-2">
							<Input
								id="password"
								name="password"
								type="password"
								autocomplete="current-password"
								required
								value={password}
								focusColor='yellow-400'
								onChange={e => setPassword(e.target.value)}
							/>
						</div>
					</div>

					<div>
						<Button full onClick={submitForm}>Sign in</Button>
					</div>
				</form>

				<p class="mt-10 text-center text-sm text-gray-500">
					Not a member? &nbsp;
					<a href="" onClick={() => location.route('/register')} class="font-semibold leading-6 text-indigo-600 hover:text-indigo-500">Register</a>
				</p>
			</div>
		</div>
	)

}
