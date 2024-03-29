import { MutableRef } from 'preact/hooks'
import { useTimer } from 'react-timer-hook'

interface InnerProps {
	expiryTimestamp: Date,
	timerRef: MutableRef<any>,
	onExpire?: () => void,
}

interface Props {
	time: number,
	timerRef: MutableRef<any>,
	onExpire?: () => void,
}

export const InnerTimer = ({
	expiryTimestamp,
	timerRef,
	onExpire,
}: InnerProps) => {

	const timer = useTimer({
		expiryTimestamp,
		autoStart: true,
		onExpire,
	})

	timerRef.current = timer


	return (
		<div class="bg-teal-500 rounded-full w-24 h-24 flex justify-center items-center">
			<span class="text-5xl">{timer.totalSeconds}</span>
		</div>
	)
}

export const Timer = ({
	time = Number.MAX_SAFE_INTEGER,
	timerRef,
	onExpire,
}: Props) => {
	const expiryTime = new Date()
	expiryTime.setSeconds(expiryTime.getSeconds() + time)

	return (
		<InnerTimer expiryTimestamp={expiryTime} timerRef={timerRef} onExpire={onExpire} />
	)
}
