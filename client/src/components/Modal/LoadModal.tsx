import { h } from 'preact'

import './Modal.scss'
import { Spinner } from '../Spinner'

type Props =  {
	show: boolean,
}

export const LoadModal = ({
	show = false,
}: Props) => {
	if (!show) return null

	return (
		<div class="relative z-20">
			<div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
			<div class="fixed inset-0 z-10 w-screen overflow-y-auto">
				<div class="flex min-h-full items-end justify-center p-4 text-center items-center sm:p-0">
					<div class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg">
						<div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
							<div class="flex justify-center">
								<Spinner />
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	)
}
