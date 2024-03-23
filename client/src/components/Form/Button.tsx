import classNames from 'classnames'
import { h } from 'preact'
import { HTMLAttributes } from 'preact/compat'

import { TailwindComponentColors } from '../../types'

export type Props = HTMLAttributes<HTMLButtonElement> & TailwindComponentColors & {
	full?: boolean
}


export const Button = ({
	children,
	full,
	bgColor = 'indigo-600',
	textColor = 'white',
	hoverColor = 'indigo-700',
	focusColor = 'green-100',
	...rest
}: Props) => (
	<button
		type="submit"
		class={
			classNames(
				'flex justify-center rounded-md px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2',
				{ 'w-full': full },
				[ `bg-${bgColor}`, `text-${textColor}`, `hover:bg-${hoverColor}`, `focus-visible:outline-${focusColor}` ],
			)
		}
		{...rest}
	>
		{children}
	</button>
)
