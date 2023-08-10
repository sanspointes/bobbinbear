import { CanvasSceneObjectView } from "./CanvasSceneObject";
import { GraphicSceneObjectView } from "./GraphicSceneObjectView";
import { BaseSceneObject } from "../types/scene";
import { For, createEffect } from "solid-js";

const SCENE_OBJECT_LOOKUP = {
  "canvas": CanvasSceneObjectView,
  "graphic": GraphicSceneObjectView,
};

export const SceneObjectChildren = (
  props: Pick<BaseSceneObject, "children">,
) => {
  createEffect(() => {
    console.log('SceneObjectChildren: children changed ', props.children);
  })

  return (
    <For each={props.children}>
      {(object) => {
        console.log("SceneObjectChildren", object);
        const Component = SCENE_OBJECT_LOOKUP[object.type];
        return <Component {...object} />;
      }}
    </For>
  );
};
