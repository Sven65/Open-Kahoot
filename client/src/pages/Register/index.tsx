import { useContext, useRef, useState } from 'preact/hooks'
import { Button } from '../../components/Form/Button'
import { Input } from '../../components/Form/Input'

import { ApiContext } from '../../context/ApiContext'
import { toast } from 'react-toastify'
import { validateEmail } from '../../util/validator'
import { PasswordCheckResult } from '../../types'
import { LoadModal } from '../../components/Modal/LoadModal'

export const Register = () => {
	const [ username, setUsername ] = useState(null)
	const [ password, setPassword ] = useState(null)
	const [ email, setEmail ] = useState(null)
	const [ confirmPassword, setConfirmPassword ] = useState(null)
	const [ isSubmitted, setIsSubmitted ] = useState<boolean>(false)
	const [ passwordFeedback, setPasswordFeedback ] = useState<PasswordCheckResult>({ feedback: { suggestions: [] } })

	const formRef = useRef()

	const [ invalidFields, setInvalidFields ] = useState<string[]>([])

	const apiContext = useContext(ApiContext)


	const submitForm = async () => {
		if (isSubmitted) return
		setPasswordFeedback({ feedback: { suggestions: [] } })
		
		setInvalidFields([])
		if(!formRef.current.checkValidity()) return toast.error('Form is invalid. Please check your inputs.')


		if (password !== confirmPassword) {
			setInvalidFields([ 'password', 'confirmPassword' ])
			return toast.error('Passwords don\'t match.')
		}

		if (!validateEmail(email)) return toast.error('Email is invalid.')
		setIsSubmitted(true)

		toast.info('Creating user...')

		let res = await apiContext.createUser({
			email,
			password,
			username,
		})

		setIsSubmitted(false)

		setPasswordFeedback(res)
	}

	return (
		(
			<>
				<LoadModal show={isSubmitted} />
				<div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8 bg-white">
					<div class="sm:mx-auto sm:w-full sm:max-w-sm">
						<img class="mx-auto h-10 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600" alt="Your Company" />
						<h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">Registration</h2>
					</div>
	
					<div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
						<form class="space-y-6" ref={formRef} onSubmit={e => e.preventDefault()}>
							<div>
								<label for="username" class="block text-sm font-medium leading-6 text-gray-900">Username</label>
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
								<label for="email" class="block text-sm font-medium leading-6 text-gray-900">Email</label>
								<div class="mt-2">
									<Input
										id="email"
										name="username"
										type="email"
										autocomplete="email"
										required
										value={email}
										onChange={e => setEmail(e.target.value)}
									/>
								</div>
							</div>

							<div>
								<label for="password" class="block text-sm font-medium leading-6 text-gray-900">Password</label>
								<div class="mt-2">
									<Input
										required
										placeholder={'Password'}
										value={password}
										onChange={(e) => setPassword(e.target.value)}
										type="password"
										error={invalidFields.includes('password')}
									/>
								</div>
								{invalidFields.includes('password') && (<p class="text-red-500 text-xs italic">Passwords don't match.</p>)}
							</div>
	
							<div>
								<label for="password" class="block text-sm font-medium leading-6 text-gray-900">Confirm Password</label>
								<div class="mt-2">
									<Input
										required
										placeholder={'Confirm Password'}
										value={confirmPassword}
										onChange={(e) => setConfirmPassword(e.target.value)}
										type="password"
										error={invalidFields.includes('confirmPassword')}
									/>
									{invalidFields.includes('confirmPassword') && (<p class="text-red-500 text-xs italic">Passwords don't match.</p>)}
								</div>
							</div>
	
							<div class="flex flex-col mt-3">
								<span class="flex text-xl text-red-600">
									{Object.keys(passwordFeedback.feedback.suggestions).length > 0 && (
										<h1>Password issues:</h1>
									)}
								</span>
								<ul>
									<li class="mt-1 text-gray-900">
										{passwordFeedback.feedback.warning && <span>{passwordFeedback.feedback.warning}</span>}
									</li>
									{passwordFeedback.feedback.suggestions.map((suggestion, i) => (
										<li class="mt-1 text-gray-900" key={i}>{suggestion}</li>
									))}
								</ul>
							</div>

							<div>
								<Button full onClick={submitForm}>Register</Button>
							</div>
						</form>

					</div>
				</div>
			</>
		)
	)

}
