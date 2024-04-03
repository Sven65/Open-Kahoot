import { useCallback, useEffect, useState } from 'preact/hooks'
import { verifyEmail } from '../../context/ApiContext'
import { useRoute } from 'preact-iso'

export const VerifyEmail = () => {
	const route = useRoute()
	
	const [ isVerified, setIsVerified ] = useState<boolean>(false)
	const [ error, setError ] = useState<string>('')

	const verify = useCallback(async () => {
		const data = await verifyEmail(route.params.id)

		if (data.error) {
			setError(data.error)
			return
		}

		setIsVerified(true)
	}, [route])

	useEffect(() => {
		verify()
	}, [verify])

	if (isVerified) {
		return (
			<main class="grid min-h-full place-items-center bg-white px-6 py-24 sm:py-32 lg:px-8">
				<div class="text-center">
					<h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">Email verified!</h1>
					<p class="mt-6 text-base leading-7 text-gray-600">You may now close this page or</p>
					<div class="mt-10 flex items-center justify-center gap-x-6">
						<a href="/" class="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Go back home</a>
					</div>
				</div>
			</main>
		)
	}

	if (error) {
		return (
			<main class="grid min-h-full place-items-center bg-white px-6 py-24 sm:py-32 lg:px-8">
				<div class="text-center">
					<h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">Verification failed</h1>
					<p class="mt-6 text-base leading-7 text-gray-600">{error}</p>
					<div class="mt-10 flex items-center justify-center gap-x-6">
						<a href="/" class="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Go back home</a>
					</div>
				</div>
			</main>
		)
	}

	return (
		<main class="grid min-h-full place-items-center bg-white px-6 py-24 sm:py-32 lg:px-8">
			<div class="text-center">
				<h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">Verifying email</h1>
				<p class="mt-6 text-base leading-7 text-gray-600">Please wait...</p>
			</div>
		</main>
	)
}
