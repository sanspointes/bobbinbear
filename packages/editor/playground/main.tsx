import { render } from 'solid-js/web'
import './styles.css';

import { Editor } from '../src';

const root = document.getElementById('root')

render(() => <Editor />, root!)
