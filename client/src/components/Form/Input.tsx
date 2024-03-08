import { h } from 'preact'
import { HTMLAttributes } from 'preact/compat'

import './Input.scss'

export type Props = HTMLAttributes<HTMLInputElement>


export const Input = ({
	children,
	...rest
}: Props) => (
	<input
		class={'ok-input'}
		{...rest}
	>
		{ children }
	</input>
)
