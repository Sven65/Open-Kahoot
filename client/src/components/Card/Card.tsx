import { PropsWithChildren } from 'preact/compat'
import './Card.scss'
import { toChildArray } from 'preact'
import classNames from 'classnames'

type Props = PropsWithChildren & {
	className?: string,
	title?: string
}

export const Card = ({
	children,
	title,
	className,
}: Props) => {
	const footer = toChildArray(children).filter(child => child.type === 'footer')
	const nonFooterChildren = toChildArray(children).filter((child) => child.type !== 'footer')

	return (
		<div class={classNames('rounded-xl border border-gray-200 bg-white py-4 px-2 shadow-md shadow-gray-100 max-h-full h-full', className)}>
			<div class="max-h-full h-full overflow-y-scroll">
				<div class="flex items-center justify-between px-2 text-base font-medium text-gray-700 overflow-y-scroll">
					<div>{title}</div>
				</div>
				<div class="border-b-2 h-2 mt-2" />
				<div class="mt-4 overflow-y-scroll">
					{nonFooterChildren}
				</div>
			</div>
			<div>
				{footer}
			</div>
		</div>
	)
}
