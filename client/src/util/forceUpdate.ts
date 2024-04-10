import { useState } from 'preact/hooks'

export const useForceUpdate = () => {
	const [ value, setValue ] = useState(0) // integer state
	return () => setValue(value => value + 1) // update state to force render
	// A function that increment 👆🏻 the previous state like here 
	// is better than directly setting `setValue(value + 1)`
}
