import { useContext } from "solid-js"
import { EmbObject } from "../types/scene"
import { AccordionItem } from "./generics/Accordian"
import { NumberInput } from "./generics/NumberInput"
import { AppContext } from "../store"
import { MoveObjectCommand, SetSceneObjectFieldCommand } from "../store/commands"
import { Point } from "@pixi/core"

type SidebarTransformProps = {
  object: EmbObject,
}
export function SidebarTransform(props: SidebarTransformProps) {
  const { dispatch } = useContext(AppContext);

  const setObjectPosition = (x: number, y: number) => {
    const obj = props.object;
    if (!obj) return;
    const newPosition = new Point(x, y);
    const cmd = new MoveObjectCommand(obj.id, newPosition);
    dispatch('scene:do-command', cmd);
  }

  return (
      <AccordionItem value="transform" header="Position and Size" innerClass="grid grid-cols-2 gap-4">
        <NumberInput 
          label="X"
          value={props.object.position.x}
          onChange={(e) => setObjectPosition(e, props.object.position.y)} 
        />
        <NumberInput 
          label="Y" 
          value={props.object.position.y}
          onChange={(e) => setObjectPosition(props.object.position.x, e)}
        />
      </AccordionItem>
  )
}
