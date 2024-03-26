import { HTMLAttributes } from 'preact/compat'

import { h } from 'preact'
import { TailwindColor, TailwindComponentColors } from '../../types'
import classNames from 'classnames'

export type Props = HTMLAttributes<HTMLInputElement> & TailwindComponentColors & {
	onEnter?: () => void
	full?: boolean,
	error?: boolean,
	placeholderColor?: TailwindColor,
	ringColor?: TailwindColor,
	errorColor?: TailwindColor,
}


export const Input = ({
	children,
	className,
	onEnter,
	full,
	bgColor,
	textColor = 'gray-900',
	hoverColor = 'indigo-700',
	focusColor = 'green-300',
	placeholderColor = 'gray-400',
	ringColor = 'gray-300',
	errorColor = 'red-400',
	error,
	...rest
}: Props) => {	
	return (
		<input
			class={
				classNames(
					'block w-full rounded-md border-0 py-1.5 shadow-sm ring-1 ring-inset focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6',
					[ `bg-${bgColor}`, `text-${textColor}`, `hover:bg-${hoverColor}`, `placeholder:text-${placeholderColor}`, error ? '' : `ring-${ringColor}`, `focus:border-${focusColor}` ],
					{ 'w-full': full, [`ring-${errorColor}`]: error },
					className,
				)
			}
			onKeyDown={
				(e) => (e.key === 'Enter' && onEnter) ? onEnter() : null
			}
			{...rest}
		/>
	)
}
