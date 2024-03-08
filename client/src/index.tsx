import { render } from 'preact'
import { LocationProvider, Router, Route } from 'preact-iso'

import { Home } from './pages/Home/index.jsx'
import { NotFound } from './pages/_404.jsx'
import './style.css'
import { Host } from './pages/Game/Host.js'
import { GameContextProvider } from './context/GameContext.js'

export function App() {
	return (
		<LocationProvider>
			<GameContextProvider>
				<Router>
					<Route path="/" component={Home} />
					<Route path="/host" component={Host} />
					<Route path="/play" component={Home} />
					<Route default component={NotFound} />
				</Router>
			</GameContextProvider>
		</LocationProvider>
	)
}

render(<App />, document.getElementById('app'))
