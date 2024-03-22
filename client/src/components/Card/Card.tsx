import { Children, PropsWithChildren } from 'preact/compat'
import './Card.scss'
import { toChildArray } from 'preact'

type Props = PropsWithChildren & {
	className?: string,
}

export const Card = ({
	children,
	className,
}: Props) => {
	const footer = toChildArray(children).filter(child => child.type === 'footer')
	const nonFooterChildren = toChildArray(children).filter((child) => child.type !== 'footer')
	

	console.log('footerr', footer)

	return (
		<div class={`card ${className}`}>
			<div class="card-inner">
				{nonFooterChildren}
			</div>
			{footer}
		</div>
	)
}
