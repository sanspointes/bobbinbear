import { Index, useContext } from "solid-js";
import { BaseSceneObject, SceneObject } from "../types/scene";
import { CanvasSceneObjectView } from "./CanvasSceneObject";
import { GraphicSceneObjectView } from "./GraphicSceneObjectView";
import { NodeSceneObjectView } from "./NodeSceneObjectView";
import { GroupSceneObjectView } from "./GroupSceneObjectView";
import { AppContext } from "../store";

const SCENE_OBJECT_LOOKUP = {
  "canvas": CanvasSceneObjectView,
  "graphic": GraphicSceneObjectView,
  "node": NodeSceneObjectView,
  "group": GroupSceneObjectView,
};

export const SceneObjectChildren = (
  props: Pick<BaseSceneObject, "children">,
) => {
  const { sceneStore } = useContext(AppContext);
  return (
    <Index each={props.children}>
      {(object, i) => {
        // eslint-disable-next-line solid/reactivity
        const o = sceneStore.objects.get(object()) as SceneObject;
        if (!o) return null;
        console.log("SceneObjectChildren", o);
        const Component = SCENE_OBJECT_LOOKUP[o.type];
        return <Component {...o} order={i} />;
      }}
    </Index>
  );
};
