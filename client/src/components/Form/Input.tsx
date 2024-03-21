import { HTMLAttributes } from 'preact/compat'

import './Input.scss'
import { h } from 'preact'

export type Props = HTMLAttributes<HTMLInputElement> & {
	onEnter?: () => void
	labelClass?: string,
	label: h.JSX.Element,
	flex?: boolean,
	full?: boolean,
}


export const Input = ({
	children,
	onEnter,
	label,
	labelClass,
	type,
	full,
	flex,
	...rest
}: Props) => {	
	if (flex) {
		return (
			<fieldset class="ok-input-container">
				{label && <label class={`ok-label ${labelClass}`}>{label}</label>}
				<input
					class={`ok-input ${label && 'has-label'} ${full ? 'full-width' : ''}`}
					type={type}
					onKeyDown={
						(e) => (e.key === 'Enter' && onEnter) ? onEnter() : null
					}
					onInvalid={e => console.log('oninv', e)}

					{...rest}
				>
					{ children }
				</input>
			</fieldset>
		)
	}
	
	return (
		<>
			{label && <label class={`ok-label ${labelClass}`}>{label}</label>}
			<input
				class={`ok-input ${label && 'has-label'} ${full ? 'full-width' : ''}`}
				type={type}
				onKeyDown={
					(e) => (e.key === 'Enter' && onEnter) ? onEnter() : null
				}
				{...rest}
			>
				{ children }
			</input>
		</>
	)
}
