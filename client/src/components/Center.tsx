import { PropsWithChildren } from 'preact/compat'
import './Center.scss'
import classNames from 'classnames'

type Props = PropsWithChildren & {
	horizontal?: boolean,
	vertical?: boolean,
}

export const Center = ({
	children,
	horizontal,
	vertical,
}: Props) => {
	return (
		<div class={classNames('center', { horizontal, vertical })}>
			{children}
		</div>
	)
}
