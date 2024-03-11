import { render } from 'preact'
import { LocationProvider, Router, Route } from 'preact-iso'

import { Home } from './pages/Home/index.jsx'
import { NotFound } from './pages/_404.jsx'
import './style.css'
import { Host } from './pages/Game/Host.js'
import { GameContextProvider } from './context/GameContext.js'
import { Player } from './pages/Game/Player.js'
import { ToastContainer } from 'react-toastify'

import 'react-toastify/dist/ReactToastify.css'

export function App() {
	return (
		<LocationProvider>
			<GameContextProvider>
				<ToastContainer />
				<Router>
					<Route path="/" component={Home} />
					<Route path="/host" component={Host} />
					<Route path="/play" component={Player} />
					<Route default component={NotFound} />
				</Router>
			</GameContextProvider>
		</LocationProvider>
	)
}

render(<App />, document.getElementById('app'))
