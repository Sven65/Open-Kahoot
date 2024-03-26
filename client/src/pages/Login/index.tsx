import { useContext, useRef, useState } from 'preact/hooks'
import { Button } from '../../components/Form/Button'
import { Input } from '../../components/Form/Input'

import { ApiContext } from '../../context/ApiContext'
import { toast } from 'react-toastify'

import './Login.scss'
import { useLocation } from 'preact-iso'

export const Login = () => {
	const [ username, setUsername ] = useState(null)
	const [ password, setPassword ] = useState(null)

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

	return (
		<div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8 bg-white">
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
								<a href="#" class="font-semibold text-indigo-600 hover:text-indigo-500">Forgot password?</a>
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
