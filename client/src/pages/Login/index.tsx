import { useContext, useRef, useState } from 'preact/hooks'
import { Card } from '../../components/Card/Card'
import { Center } from '../../components/Center'
import { Button } from '../../components/Form/Button'
import { Input } from '../../components/Form/Input'

import { ApiContext } from '../../context/ApiContext'
import { toast } from 'react-toastify'

export const Login = () => {
	const [ username, setUsername ] = useState(null)
	const [ password, setPassword ] = useState(null)

	const formRef = useRef()

	const apiContext = useContext(ApiContext)


	const submitForm = () => {
		if(!formRef.current.checkValidity()) return toast.error('Form is invalid. Please check your inputs.')

		apiContext.login(
			username,
			password,
		)
	}

	return (
		<Center horizontal vertical>
			<Card className="register-card">
				<h1>Login</h1>
				<form class="register-form" ref={formRef} onSubmit={e => e.preventDefault()}>
					<Input required placeholder={'Username'} label="Username" flex full value={username} onChange={(e) => setUsername(e.target.value)} />
					<Input required placeholder={'Password'} label="Password" flex full value={password} onChange={(e) => setPassword(e.target.value)} type="password" />
					<Button color='green' full onClick={submitForm}>Login</Button>
				</form>
			</Card>
		</Center>
	)
}
