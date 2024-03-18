import { h } from 'preact'
import { HTMLAttributes } from 'preact/compat'

import './Input.scss'

export type Props = HTMLAttributes<HTMLInputElement> & {
	onEnter?: () => void
	labelClass?: string,
}


export const Input = ({
	children,
	onEnter,
	label,
	labelClass,
	type,
	...rest
}: Props) => {
	return (
		<>
			{label && <label class={`ok-label ${labelClass}`}>{label}</label>}
			<input
				class={`ok-input ${label && 'has-label'}`}
				type={type}
				onKeyDown={
					(e) => (e.key === 'Enter' && onEnter) ? onEnter() : null}
				{...rest}
			>
				{ children }
			</input>
		</>
	)
}
