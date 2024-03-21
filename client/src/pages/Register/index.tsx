import { useContext, useRef, useState } from 'preact/hooks'
import { Card } from '../../components/Card/Card'
import { Center } from '../../components/Center'
import { Button } from '../../components/Form/Button'
import { Input } from '../../components/Form/Input'

import './Register.scss'
import { ApiContext } from '../../context/ApiContext'
import { toast } from 'react-toastify'
import { validateEmail } from '../../util/validator'

export const Register = () => {
	const [ username, setUsername ] = useState(null)
	const [ password, setPassword ] = useState(null)
	const [ email, setEmail ] = useState(null)
	const [ confirmPassword, setConfirmPassword ] = useState(null)
	const formRef = useRef()

	const apiContext = useContext(ApiContext)


	const submitForm = () => {
		if(!formRef.current.checkValidity()) return toast.error('Form is invalid. Please check your inputs.')

		if (password !== confirmPassword) return toast.error('Passwords don\'t match.')

		if (!validateEmail(email)) return toast.error('Email is invalid.')

		apiContext.createUser({
			email,
			password,
			username,
		})
	}

	return (
		<Center horizontal vertical>
			<Card className="register-card">
				<h1>Register</h1>
				<form class="register-form" ref={formRef} onSubmit={e => e.preventDefault()}>
					<Input required placeholder={'Username'} label="Username" flex full value={username} onChange={(e) => setUsername(e.target.value)} />
					<Input required placeholder={'Email'}    label="Email"    flex full value={email}    onChange={(e) => setEmail(e.target.value)} type="email" />
					<Input required placeholder={'Password'} label="Password" flex full value={password} onChange={(e) => setPassword(e.target.value)} type="password" />
					<Input required placeholder={'Confirm Password'} label="Confirm Password" flex full value={confirmPassword} onChange={(e) => setConfirmPassword(e.target.value)} type="password" />
					<Button color='green' full onClick={submitForm}>Register</Button>
				</form>
			</Card>
		</Center>
	)
}
