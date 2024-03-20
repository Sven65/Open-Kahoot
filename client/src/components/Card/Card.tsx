import { PropsWithChildren } from 'preact/compat'
import './Card.scss'

type Props = PropsWithChildren & {
	className?: string,
}

export const Card = ({
	children,
	className,
}: Props) => {
	return (
		<div class={`card ${className}`}>
			<div class="card-inner">
				{children}
			</div>
		</div>
	)
}
