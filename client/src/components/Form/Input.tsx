import { h } from 'preact'
import { HTMLAttributes } from 'preact/compat'

import './Input.scss'

export type Props = HTMLAttributes<HTMLInputElement> & {
	onEnter?: () => void
}


export const Input = ({
	children,
	onEnter,
	...rest
}: Props) => (
	<input
		class={'ok-input'}
		onKeyDown={(e) => (e.key === 'Enter' && onEnter) ? onEnter() : null}
		{...rest}
	>
		{ children }
	</input>
)
