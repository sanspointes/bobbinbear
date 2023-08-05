
import { useContext } from "solid-js"
import * as API from "../store/api"
import { Button } from "./generics/Button"
import { AppContext } from "../store"

export const Toolbar = () => {
  const { toolStore, dispatch } = useContext(AppContext);
  return (
    <div class="flex p-2">
      <Button onClick={() => API.createCanvas(dispatch)}>New Canvas</Button>
      {toolStore.currentCursor}
    </div>
  )
}
