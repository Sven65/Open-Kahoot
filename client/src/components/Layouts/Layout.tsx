import classNames from 'classnames'
import { PropsWithChildren } from 'preact/compat'

type Props = PropsWithChildren & {
	className?: string
}

export const Layout = ({
	children,
	className,
}: Props) => {
	return (
		<div
			class={classNames(className, 'flex bg-gray-100 text-gray-900 max-h-full h-full')}
		>
			{children}
		</div>
	)
}
