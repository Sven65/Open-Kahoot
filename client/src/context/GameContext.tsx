import { createContext } from 'preact'
import { StateUpdater, useState } from 'preact/hooks'
import { io } from 'socket.io-client'
import { SocketEvents } from '../types'

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
	const [ roomId, setRoomId ] = useState('')

	socket.on(SocketEvents.RoomCreated, (roomCode: string) => {
		console.log('room_code', roomCode)
		setRoomId(roomCode)
	})

	socket.on(SocketEvents.RoomJoined, (roomCode: string) => {
		console.log('Room joined successfully', roomCode)
		setRoomId(roomCode)
	})


	socket.on(SocketEvents.JoinFailed, (data) => {
		console.log('join fail', data)
	})

	return (
		<GameContext.Provider value={{
			join: (room: string) => {
				console.log('joining room', room)
				socket.emit(SocketEvents.Join, room)
			},
			roomId: [ roomId, setRoomId ],
			createRoom: () => {
				socket.emit(SocketEvents.CreateRoom)
			},
		}}>
			{children}
		</GameContext.Provider>
	)
}