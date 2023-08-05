import { For } from "solid-js";
import { CanvasSceneObjectView } from "./CanvasSceneObject";
import { GraphicSceneObjectView } from "./GraphicSceneObjectView";
import { BaseSceneObject } from "../types/scene";

const SCENE_OBJECT_LOOKUP = {
  'canvas': CanvasSceneObjectView,
  'graphic': GraphicSceneObjectView,
}

export const SceneObjectChildren = (props: Pick<BaseSceneObject, 'children'>) => {
  return (
    <For each={props.children}>
      {object => {
          const Component = SCENE_OBJECT_LOOKUP[object.type];
          return <Component {...object} />
        }
      }
    </For>
  )
}
