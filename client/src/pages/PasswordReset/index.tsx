import { useContext, useRef, useState } from 'preact/hooks'
import { Button } from '../../components/Form/Button'
import { Input } from '../../components/Form/Input'

import { ApiContext, resetPassword } from '../../context/ApiContext'
import { toast } from 'react-toastify'

import { useLocation, useRoute } from 'preact-iso'

export const ResetPassword = () => {
	const [ password, setPassword ] = useState(null)
	const [ confirmPassword, setConfirmPassword ] = useState(null)
	const [ invalidFields, setInvalidFields ] = useState<string[]>([])


	const formRef = useRef()
	const route = useRoute()
	const location = useLocation()


	const submitForm = async () => {
		setInvalidFields([])

		if(!formRef.current.checkValidity()) return toast.error('Form is invalid. Please check your inputs.')


		if (password !== confirmPassword) {
			setInvalidFields([ 'password', 'confirmPassword' ])
			return toast.error('Passwords don\'t match.')
		}
		toast.info('Changing password...')

		let res = await resetPassword(route.params.id, password)


		if (res.error) {
			toast.error(res.error)
			return
		}
		
		toast.success('Password changed!')
		setTimeout(() => location.route('/@me'), 2000)
	}

	return (
		<div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8 bg-white">
			<div class="sm:mx-auto sm:w-full sm:max-w-sm">
				<img class="mx-auto h-10 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600" alt="Your Company" />
				<h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">Reset your password</h2>
			</div>

			<div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
				<form class="space-y-6" ref={formRef} onSubmit={e => e.preventDefault()}>
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
	
					<div>
						<Button full onClick={submitForm}>Change</Button>
					</div>
				</form>

			</div>
		</div>
	)

}
