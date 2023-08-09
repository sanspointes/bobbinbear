import { useContext } from "solid-js"
import { AiOutlineSelect } from 'solid-icons/ai'

import * as API from "../store/api"
import { Button } from "./generics/Button"
import { AppContext } from "../store"
import { Tool } from "../store/toolStore"

export const Toolbar = () => {
  const { dispatch } = useContext(AppContext);
  return (
    <div class="flex p-2 gap-2">
      <Button onClick={() => API.createCanvas(dispatch)}>New Canvas</Button>
      <Button onClick={() => dispatch('tool:switch', Tool.Select)}><AiOutlineSelect /></Button>
    </div>
  )
}
