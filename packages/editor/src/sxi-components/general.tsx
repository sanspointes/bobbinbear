import { For, createEffect } from "solid-js";
import { BaseSceneObject } from "../types/scene";
import { CanvasSceneObjectView } from "./CanvasSceneObject";
import { GraphicSceneObjectView } from "./GraphicSceneObjectView";
import { NodeSceneObjectView } from "./NodeSceneObjectView";
import { GroupSceneObjectView } from "./GroupSceneObjectView";

const SCENE_OBJECT_LOOKUP = {
  "canvas": CanvasSceneObjectView,
  "graphic": GraphicSceneObjectView,
  "node": NodeSceneObjectView,
  "group": GroupSceneObjectView,
};

export const SceneObjectChildren = (
  props: Pick<BaseSceneObject, "children">,
) => {
  return (
    <For each={props.children}>
      {(object, i) => {
        console.log("SceneObjectChildren", object);
        const Component = SCENE_OBJECT_LOOKUP[object.type];
        return <Component {...object} order={i()} />;
      }}
    </For>
  );
};
