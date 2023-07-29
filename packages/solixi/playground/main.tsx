/* @jsxImportSource solid-js */

import { render } from 'solid-js/web';
import { Canvas } from '../src'
import { Scene } from './Scene'

const App = () => {
  return <Canvas devtools={true}>
    <Scene/>
  </Canvas>
}

render(() => <App />, document.getElementById('root')!);
