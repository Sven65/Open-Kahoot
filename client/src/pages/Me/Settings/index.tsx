import { useContext } from 'preact/hooks'
import { RequireLogin } from '../../../components/HoC/RequireLogin'
import { ApiContext } from '../../../context/ApiContext'
import { Button } from '../../../components/Form/Button'
import { DashboardLayout } from '../../../components/Layouts/Dashboard/Dashboard'
import { Card } from '../../../components/Card/Card'
import { Input } from '../../../components/Form/Input'
  
const Settings = () => {
	const apiContext = useContext(ApiContext)

	if (!apiContext.user) return <h1>Please wait, logging in.</h1>

	return (
		<DashboardLayout>
			<div class="flex flex-col flex-1 h-full">
				<Card title="User Settings" className='flex-1'>
					<div>
					Your email address is <span class="font-bold">email@example.com</span>
						<br />
						<a href="#" class="text-blue-500 underline">Change</a>
					</div>
					<hr />
					<div>
						Avatar

						<div class="w-[256px] h-[256px]">
							<img src={apiContext.getAvatarUrl()} alt={apiContext.user.username} />
							<div class="rounded-full bg-green-200 hover:bg-green-100 hover:cursor-pointer w-16 h-16 flex justify-center items-center relative -top-8 -right-[224px]">
								<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
									<path strokeLinecap="round" strokeLinejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10" />
								</svg>
							</div>
						</div>
					</div>
				</Card>
				<Card title="Password" className='flex-1'>
					<div>
						<label for="maxPoints" class="block text-sm font-medium leading-6 text-gray-900">Current password</label>
						<div class="mt-2">
							<Input />
						</div>
					</div>
					<div>
						<label for="maxPoints" class="block text-sm font-medium leading-6 text-gray-900">New password</label>
						<div class="mt-2">
							<Input />
						</div>
					</div>
					<Button className="mt-2" full>Change</Button>
				</Card>
				<Card title="Delete account" className='flex-1'>
					<span class="inline-flex items-center rounded-md bg-red-50 px-2 py-1 text-md m-2 font-medium text-red-700 ring-1 ring-inset ring-red-600/10">Proceed with caution!</span>
					<br />

					Make sure you have taken backup of your account in case you ever need to get access to your data. We will completely wipe your data. There is no way to access your account after this action.
					<br />

					<a href="#" class="text-red-600 underline">Continue with deletion</a>
				</Card>
			
			</div>
		</DashboardLayout>
	)
}

export const SettingsPage = RequireLogin(Settings)
