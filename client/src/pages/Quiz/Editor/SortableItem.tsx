import { h } from 'preact'
import { useSortable } from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import { Question } from '../../../types'

interface Props {
	id: any,
	question: Question
}

export const SortableItem = ({
	id,
	question,
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
		<div ref={setNodeRef} style={style} {...attributes} {...listeners} key={question.id}>
			<h1>{question.question}</h1>
		</div>
	)
}
