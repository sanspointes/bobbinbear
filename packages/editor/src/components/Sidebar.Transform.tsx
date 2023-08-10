import { useContext } from "solid-js"
import { SceneObject } from "../types/scene"
import { AccordionItem } from "./generics/Accordian"
import { NumberInput } from "./generics/NumberInput"
import { AppContext } from "../store"
import { SetSceneObjectFieldCommand } from "../store/commands/object"
import { Point } from "@pixi/core"

type SidebarTransformProps = {
  object: SceneObject,
}
export function SidebarTransform(props: SidebarTransformProps) {
  const { dispatch } = useContext(AppContext);

  const setObjectPosition = (x: number, y: number) => {
    const obj = props.object;
    if (!obj) return;
    const newPosition = new Point(x, y);
    const cmd = new SetSceneObjectFieldCommand(obj.id, 'position', newPosition);
    dispatch('scene:do-command', cmd);
  }

  return (
      <AccordionItem value="pos-and-size" header="Position and Size" innerClass="grid grid-cols-2 gap-4">
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
