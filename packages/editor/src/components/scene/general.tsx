import { For } from "solid-js";
import { CanvasSceneObjectView } from "./CanvasSceneObject";
import { BaseSceneObject } from "../../store/scene";
import { GraphicSceneObjectView } from "./GraphicSceneObjectView";

const SCENE_OBJECT_LOOKUP = {
  'canvas': CanvasSceneObjectView,
  'graphic': GraphicSceneObjectView,
}

export const SceneObjectChildren = (props: Pick<BaseSceneObject, 'children'>) => {
  return (
    <For each={props.children}>
      {object => {
          const Component = SCENE_OBJECT_LOOKUP[object.type];
          // @ts-expect-error ; Can't be bothered to add typing for component
          return <Component {...object} />
        }
      }
    </For>
  )
}
