import { h } from 'preact'
import { HTMLAttributes } from 'preact/compat'

import './Button.scss'

export type Props = HTMLAttributes<HTMLButtonElement> & {
	color: 'red' | 'green',
	full?: boolean
}


export const Button = ({
	children,
	color,
	full,
	...rest
}: Props) => (
	<button
		class={`ok-button button-${color} ${full ? 'btn-full' : ''}`}
		{...rest}
	>
		{ children }
	</button>
)
