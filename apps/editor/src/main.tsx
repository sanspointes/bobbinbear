/** @jsxImportSource solid-js */
import { ErrorBoundary } from 'solid-js';
import { render } from 'solid-js/web';
import './styles.css';
//
// import { attachDevtoolsOverlay } from '@solid-devtools/overlay'
// attachDevtoolsOverlay()

import { Editor } from './Editor';
import { ErrorView } from './components/ErrorView';

const root = document.getElementById('root')

const App = () => {
  return (
    <ErrorBoundary fallback={error => <ErrorView error={error}/>}>
      <Editor />
    </ErrorBoundary>
  )
}
render(() => <App />, root!)
