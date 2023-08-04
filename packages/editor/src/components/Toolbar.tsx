
import * as API from "../store/api"
import { Button } from "./generics/Button"

export const Toolbar = () => {
  return (
    <div class="flex p-2">
      <Button onClick={() => API.createCanvas()}>New Canvas</Button>
    </div>
  )
}
