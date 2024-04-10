import { useContext } from 'preact/hooks'
import { RequireLogin } from '../../../components/HoC/RequireLogin'
import { ApiContext } from '../../../context/ApiContext'
import { useLocation } from 'preact-iso'
import { Button } from '../../../components/Form/Button'
import { GameContext } from '../../../context/GameContext'
import { DashboardLayout } from '../../../components/Layouts/Dashboard/Dashboard'
import { InputModal } from '../../../components/Modal/InputModal'
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
