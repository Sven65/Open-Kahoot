import { useContext } from 'preact/hooks'
import { RequireLogin } from '../../components/HoC/RequireLogin'
import { ApiContext } from '../../context/ApiContext'

const InternalMe = () => {
	const apiContext = useContext(ApiContext)
	console.log('apiContext.user', apiContext.user)

	if (!apiContext.user) return <h1>Please wait, logging in.</h1>

	return (
		<h1>Welcome, {apiContext.user.username}</h1>
	)
}

export const Me = RequireLogin(InternalMe)
