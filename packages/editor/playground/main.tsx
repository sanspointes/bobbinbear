import { render } from 'solid-js/web'
import './styles.css';

import * as dev from 'solid-devtools'
console.log(dev);

import { Editor } from '../src';

const root = document.getElementById('root')

render(() => <Editor />, root!)
