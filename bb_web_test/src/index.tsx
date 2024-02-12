/* @refresh reload */
import { ErrorBoundary, render } from 'solid-js/web'

import './assets/style.css'
import App from './App'
import { ErrorView } from './features/error'

const root = document.getElementById('root')

render(() => (
    <ErrorBoundary fallback={error => <ErrorView error={error} />}>
        <App />
    </ErrorBoundary>
), root!)
