import { useEffect } from 'preact/hooks'
import { useTimer } from 'react-timer-hook'

interface InnerProps {
	expiryTimestamp: Date,
}

interface Props {
	time: number
}

export const InnerTimer = ({
	expiryTimestamp,
}: InnerProps) => {
	const {
		totalSeconds,
	} = useTimer({
		expiryTimestamp,
		autoStart: true,
	})

	return (
		<div>
			Seconds left: {totalSeconds}
		</div>
	)
}

export const Timer = ({
	time = 0,
}: Props) => {
	const expiryTime = new Date()
	expiryTime.setSeconds(expiryTime.getSeconds() + time)

	return (
		<InnerTimer expiryTimestamp={expiryTime} />
	)
}
