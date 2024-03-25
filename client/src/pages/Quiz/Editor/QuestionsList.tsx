import { useEffect, useState } from 'preact/hooks'
import { Question } from '../../../types'
import { DndContext, KeyboardSensor, PointerSensor, closestCenter, useSensor, useSensors } from '@dnd-kit/core'
import { SortableContext, arrayMove, sortableKeyboardCoordinates, verticalListSortingStrategy } from '@dnd-kit/sortable'
import { SortableItem } from './SortableItem'

interface Props {
	questions: Question[],
	onEdit: (newQuestions: Question[]) => void
	onAddQuestion: () => void,
	onClickQuestion: (id: string) => void,
}

export const QuestionsList = ({
	questions,
	onEdit,
	onClickQuestion,
	onAddQuestion,
}: Props) => {
	const [ items, setItems ] = useState(questions)

	useEffect(() => {
		onEdit(items)
	}, [items])

	useEffect(() => {
		setItems(questions)
	}, [questions])

	const sensors = useSensors(
		useSensor(PointerSensor),
		useSensor(KeyboardSensor, {
			coordinateGetter: sortableKeyboardCoordinates,
		}),
		useSensor(PointerSensor, {
			activationConstraint: {
			  distance: 8,
			},
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
				<div class="rounded-xl border border-gray-200 bg-white py-4 px-2 shadow-md shadow-gray-100">
					<div class="flex items-center justify-between px-2 text-base font-medium text-gray-700">
						<div>Questions</div>
						<div>
							<button class="flex h-8 w-8 items-center justify-center rounded-full bg-gray-200 text-black">
								<svg class="h-5 w-5" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
									<path d="M6 18L18 6M6 6l12 12" stroke-linecap="round" stroke-linejoin="round" />
								</svg>
							</button>
						</div>
					</div>
					<div class="mt-4">
						<div class="flex w-full flex-col overflow-y-scroll">
							{items.map(question => 
								(
									<SortableItem id={question.id} key={`sortable-${question.id}`}>
										<div class="group flex items-center gap-x-5 rounded-md px-2.5 py-2 transition-all duration-75 hover:bg-green-100"  onClick={() => onClickQuestion(question.id)}>
											<div class="flex h-12 w-12 items-center rounded-lg bg-gray-200 text-black group-hover:bg-green-200">
												<span class="tag w-full text-center text-2xl font-medium text-gray-700 group-hover:text-green-900">
													<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="mx-auto w-6 h-6">
														<path strokeLinecap="round" strokeLinejoin="round" d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 5.25h.008v.008H12v-.008Z" />
													</svg>

												</span>
											</div>
											<div class="flex flex-col items-start justify-between font-light text-gray-600">
												<p class="text-[15px]">{question.question}</p>
												<span class="text-xs font-light text-gray-400" />
											</div>
										</div>
									</SortableItem>
								),
							)}
						</div>
					</div>
					<div class="mx-auto flex items-center justify-start gap-x-3 rounded-lg border border-lime-500 bg-lime-500 px-4 py-2" onClick={onAddQuestion}>
						<button class="inline-flex items-center space-x-2 rounded-full bg-transparent font-semibold text-white">
							<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
								<path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
							</svg>
						</button>
						<p class="text-sm text-white">Add new</p>
					</div>
				</div>
			</SortableContext>
		</DndContext>
	)
}
