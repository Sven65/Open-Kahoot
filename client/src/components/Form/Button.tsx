import { h } from 'preact'
import { HTMLAttributes } from 'preact/compat'

import './Button.scss'

export type Props = HTMLAttributes<HTMLButtonElement> & {
	color: 'red' | 'green'
}


export const Button = ({
	children,
	color,
	...rest
}: Props) => (
	<button
		class={`ok-button button-${color}`}
		{...rest}
	>
		{ children }
	</button>
)
