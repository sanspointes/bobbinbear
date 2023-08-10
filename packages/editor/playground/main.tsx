import { attachDevtoolsOverlay } from '@solid-devtools/overlay'
import { render } from 'solid-js/web'
import './styles.css';

import 'solid-devtools'

attachDevtoolsOverlay()

import { Editor } from '../src';

const root = document.getElementById('root')

render(() => <Editor />, root!)
