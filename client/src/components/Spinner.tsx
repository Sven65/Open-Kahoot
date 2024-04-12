import { TailwindColor } from '../types'

type Props = {
	border?: TailwindColor,
	color?: TailwindColor
}

export const Spinner = ({
	border = 'gray-300',
	color = 'blue-600',
}: Props) => (
	<div class={`border-${border} h-20 w-20 animate-spin rounded-full border-8 border-t-${color}`} />
)
