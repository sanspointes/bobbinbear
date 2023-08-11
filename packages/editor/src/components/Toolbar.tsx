import { useContext } from "solid-js"
import { TbPointer } from 'solid-icons/tb'
import { CgSquare } from 'solid-icons/cg'

import * as API from "../store/api"
import { Button } from "./generics/Button"
import { AppContext } from "../store"
import { Tool } from "../store/toolStore"

export const Toolbar = () => {
  const { dispatch, toolStore } = useContext(AppContext);
  return (
    <div class="flex p-2 gap-2 border-b border-yellow-500 bg-yellow-400 border-solid">
      <Button onClick={() => API.createCanvas(dispatch)}>New Canvas</Button>
      <Button onClick={() => dispatch('tool:switch', Tool.Select)}><TbPointer /></Button>
      <Button onClick={() => dispatch('tool:switch', Tool.Box)}><CgSquare /></Button>
    </div>
  )
}
