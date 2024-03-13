import { useEffect, useState } from 'preact/hooks'
import { Question } from '../../../types'
import { DndContext, KeyboardSensor, PointerSensor, closestCenter, useSensor, useSensors } from '@dnd-kit/core'
import { SortableContext, arrayMove, sortableKeyboardCoordinates, verticalListSortingStrategy } from '@dnd-kit/sortable'
import { SortableItem } from './SortableItem'

interface Props {
	questions: Question[],
	onEdit: (newQuestions: Question[]) => void
}

export const QuestionEditor = ({
	questions,
	onEdit,
}: Props) => {
	const [ items, setItems ] = useState(questions)

	useEffect(() => {
		onEdit(items)
	}, [items])

	const sensors = useSensors(
		useSensor(PointerSensor),
		useSensor(KeyboardSensor, {
			coordinateGetter: sortableKeyboardCoordinates,
		}),
	)

	const recalculateRanks = (questions: Question[]) => {
		return questions.map((question, idx) => ({
			...question,
			question_rank: idx + 1,
		}))
	}

	const handleDragEnd = (event) => {
		const { active, over } = event
    
		if (active.id !== over.id) {
			setItems((items) => {
				const oldIndex = items.findIndex(item => item.id == active.id)
				const newIndex = items.findIndex(item => item.id == over.id)

				return recalculateRanks(arrayMove(items, oldIndex, newIndex))
			})
		}
	}

	console.log('items', items)

	return (
		<DndContext
			sensors={sensors}
			collisionDetection={closestCenter}
			onDragEnd={handleDragEnd}
		>
			<SortableContext
				items={items}
				strategy={verticalListSortingStrategy}
			>
				{items.map(question => <SortableItem question={question} id={question.id} />)}
			</SortableContext>
		</DndContext>
	)
}
