import { ComponentChildren } from 'preact'

import './Modal.scss'

type Props =  {
	show: boolean,
	onClose?: () => void,
	children?: ComponentChildren,
}

export const Modal = ({
	show = false,
	onClose,
	children,
}: Props) => {
	return (
		<div class={`modal ${!show && 'hide'}`}>
			<div class="modal-content">
				<div class="modal-header">
					<span onClick={onClose}>X</span>
				</div>

				{children}
			</div>
		</div>
	)
}
