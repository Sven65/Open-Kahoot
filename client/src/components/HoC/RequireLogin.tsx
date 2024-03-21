import { h } from 'preact'
import { useContext, useEffect } from 'preact/hooks'

import { getCookie } from '../../util/cookies'
import { ApiContext } from '../../context/ApiContext'

export const RequireLogin = (WrappedComponent: any) => {
	const session = getCookie('login_session')

	if (!session) {
		const newComponent = () => {
			window.location.href = '/login'

			return (
				<div>
					<h1>Not logged in</h1>
				</div>
			)
		}

		return newComponent
	}

	const newComponent = (props) => {
		// eslint-disable-next-line react-hooks/rules-of-hooks
		const apiContext = useContext(ApiContext)

		// eslint-disable-next-line react-hooks/rules-of-hooks
		useEffect(() => {
			apiContext.fetchMe()
		}, [])
		
		return (
			<WrappedComponent {...props} />
		)
	}

	return newComponent
}
