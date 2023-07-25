import { render } from 'solid-js/web'
import { App } from './App'
import { Portal as ThreePortal } from '@solid-three/fiber'

import 'solid-devtools'

render(() => <App />, document.getElementById('root')!)
