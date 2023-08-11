/** @jsxImportSource solid-js */
import { render } from 'solid-js/web'
import './styles.css';

import { attachDevtoolsOverlay } from '@solid-devtools/overlay'
attachDevtoolsOverlay()

import { Editor } from '../src';

const root = document.getElementById('root')

render(() => <Editor />, root!)
