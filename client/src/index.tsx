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
import { Me } from './pages/Me/index.js'
import { QuizEditor2 } from './pages/Quiz/Editor/editor2.js'

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
						<Route path="/quiz/:id/edit" component={QuizEditor2} />
						<Route path="/register" component={Register} />
						<Route path="/login" component={Login} />
						<Route path="/@me" component={Me} />
						<Route default component={NotFound} />
					</Router>
				</ApiContextProvider>
			</GameContextProvider>
		</LocationProvider>
	)
}

render(<App />, document.getElementById('app'))

// On page load or when changing themes, best to add inline in `head` to avoid FOUC
if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
	document.documentElement.classList.add('dark')
} else {
	document.documentElement.classList.remove('dark')
}
  
// Whenever the user explicitly chooses light mode
localStorage.theme = 'light'
  
// Whenever the user explicitly chooses dark mode
localStorage.theme = 'dark'
  
// Whenever the user explicitly chooses to respect the OS preference
localStorage.removeItem('theme')
