import { createContext } from 'preact'
import { StateUpdater, useState } from 'preact/hooks'
import { io } from 'socket.io-client'

export type IGameContext = {
	join: (room: string) => void,
	roomId: [string, StateUpdater<String>],
	createRoom: () => void,
}

const URL = process.env.NODE_ENV === 'production' ? undefined : 'http://localhost:3000'

export const socket = io(URL)



export const GameContext = createContext<IGameContext>(null)


export const GameContextProvider = ({
	children,
}) => {
	const roomId = useState('')

	socket.on('room_created', (roomCode: string) => {
		console.log('room_code', roomCode)
		roomId[1](roomCode)
	})

	return (
		<GameContext.Provider value={{
			join: (room: string) => {
				console.log('joining room', room)
				socket.emit('join', room)
			},
			roomId,
			createRoom: () => {
				socket.emit('createRoom')
			},
		}}>
			{children}
		</GameContext.Provider>
	)
}
