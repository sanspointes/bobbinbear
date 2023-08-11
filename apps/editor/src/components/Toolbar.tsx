import { useContext } from "solid-js"
import { TbPointer } from 'solid-icons/tb'
import { CgSquare } from 'solid-icons/cg'

import * as API from "../store/api"
import { Button } from "./generics/Button"
import { AppContext } from "../store"
import { Tool } from "../store/toolStore"

export const Toolbar = () => {
  const app = useContext(AppContext);
  return (
    <div class="flex p-2 gap-2 border-b border-yellow-500 bg-yellow-400 border-solid">
      <Button onClick={() => API.createCanvas(app.dispatch)}>New Canvas</Button>
      <Button variant={app.toolStore.tool === Tool.Select ? "highlighted" : "default"} onClick={() => app.dispatch('tool:switch', Tool.Select)}><TbPointer /></Button>
      <Button variant={app.toolStore.tool === Tool.Box ? "highlighted" : "default"} onClick={() => app.dispatch('tool:switch', Tool.Box)}><CgSquare /></Button>
    </div>
  )
}
