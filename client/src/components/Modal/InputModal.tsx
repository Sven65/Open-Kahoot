import { h } from 'preact'
import { ComponentChildren } from 'preact'

import './Modal.scss'
import { Input } from '../Form/Input'
import { useState } from 'preact/hooks'

type Props =  {
	show: boolean,
	title?: string,
	onClose?: () => void,
	onAction?: (value: string) => void,
	children?: ComponentChildren,
	text?: string
	actionText?: string
	placeholder?: string,
	icon?: h.JSX.Element,
}

export const InputModal = ({
	show = false,
	title,
	text,
	actionText,
	placeholder,
	icon,
	onAction,
	onClose,
}: Props) => {
	const [ value, setValue ] = useState('')
	if (!show) return null

	return (
		<div class="relative z-20">
			<div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
			<div class="fixed inset-0 z-10 w-screen overflow-y-auto">
				<div class="flex min-h-full items-end justify-center p-4 text-center items-center sm:p-0">
					<div class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg">
						<div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
							<div class="sm:flex sm:items-start">
								<div class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-green-100 sm:mx-0 sm:h-10 sm:w-10">
									<svg class="h-6 w-6 text-green-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
										{icon}
									</svg>
								</div>
								<div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
									<h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">{title}</h3>
									<div class="mt-2">
										<p class="text-sm text-gray-500">{text}</p>
										<Input
											placeholder={placeholder}
											value={value}
											onChange={e => setValue(e.target.value)}
											onKeyDown={e => {e.key === 'Enter' ? onAction(value) : null}}
										/>
									</div>
								</div>
							</div>
						</div>
						<div class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
							<button type="button" class="inline-flex w-full justify-center rounded-md bg-green-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-green-500 sm:ml-3 sm:w-auto" onClick={() => onAction(value)}>{actionText}</button>
							<button type="button" class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:mt-0 sm:w-auto" onClick={onClose}>Cancel</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	)
}
