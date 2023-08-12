/** @jsxImportSource solid-js */
import { ErrorBoundary, Show, render } from 'solid-js/web'
import './styles.css';

import { attachDevtoolsOverlay } from '@solid-devtools/overlay'
attachDevtoolsOverlay()

import { Editor } from '../src';
import { ErrorView } from '../src/components/Error';

const root = document.getElementById('root')

const App = () => {
  return (
    <ErrorBoundary fallback={error => <ErrorView error={error}/>}>
      <Editor />
    </ErrorBoundary>
  )
}
render(() => <Editor />, root!)
