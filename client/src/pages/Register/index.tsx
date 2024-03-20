import { Card } from '../../components/Card/Card'
import { Center } from '../../components/Center'
import { Button } from '../../components/Form/Button'
import { Input } from '../../components/Form/Input'

import './Register.scss'

export const Register = () => {
	return (
		<Center horizontal vertical>
			<Card className="register-card">
				<h1>Register</h1>
				<form class="register-form">
					<Input placeholder={'Username'} label="Username" flex full />
					<Input placeholder={'Email'}    label="Email"    flex full type="email" />
					<Input placeholder={'Password'} label="Password" flex full type="password" />
					<Input placeholder={'Confirm Password'} label="Confirm Password" flex full type="password" />
					<Button color='green' full>Register</Button>
				</form>
			</Card>
		</Center>
	)
}
