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
import { QuizEditor } from './pages/Quiz/Editor/editor.js'
import { ApiContextProvider } from './context/ApiContext.js'
import { Register } from './pages/Register/index.js'
import { Login } from './pages/Login/index.js'

export function App() {
	return (
		<LocationProvider>
			<GameContextProvider>
				<ApiContextProvider>
					<ToastContainer />
					<Router>
						<Route path="/" component={Home} />
						<Route path="/host" component={Host} />
						<Route path="/play" component={Player} />
						<Route path="/quiz/:id/edit" component={QuizEditor} />
						<Route path="/register" component={Register} />
						<Route path="/login" component={Login} />
						<Route default component={NotFound} />
					</Router>
				</ApiContextProvider>
			</GameContextProvider>
		</LocationProvider>
	)
}

render(<App />, document.getElementById('app'))
