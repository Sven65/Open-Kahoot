import { ComponentChildren } from 'preact'

import './CenterText.scss'

type Props = {
	children?: ComponentChildren,
	className?: string,
}

export const CenterText = ({
	className = '',
	children,
}: Props) => {
	return (
		<div class={`center-text ${className}`}>
			{children}
		</div>
	)
}
