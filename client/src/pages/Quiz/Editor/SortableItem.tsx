import { h } from 'preact'
import { useSortable } from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import { PropsWithChildren } from 'preact/compat'

type Props = PropsWithChildren & {
	id: string,
	onClick?: () => void,
}

export const SortableItem = ({
	id,
	onClick,
	children,
}: Props) => {
	const {
	  attributes,
	  listeners,
	  setNodeRef,
	  transform,
	  transition,
	} = useSortable({ id })
	
	const style = {
	  transform: CSS.Transform.toString(transform),
	  transition,
	}
	
	return (
		<div ref={setNodeRef} style={style} {...attributes} {...listeners} onClick={onClick}>
			{children}
		</div>
	)
}
