import { For, useContext } from "solid-js";
import { EmbBase, EmbObject } from "../types/scene";
import { EmbCanvasView } from "./EmbCanvas";
import { EmbVectorView } from "./EmbVector";
import { EmbNodeView } from "./EmbNode";
import { EmbGroupView } from "./EmbGroup";
import { AppContext } from "../store";

const SCENE_OBJECT_LOOKUP = {
  "canvas": EmbCanvasView,
  "graphic": EmbVectorView,
  "node": EmbNodeView,
  "group": EmbGroupView,
};

export const SceneObjectChildren = (
  props: Pick<EmbBase, "children">,
) => {
  const { sceneStore } = useContext(AppContext);
  return (
    <For each={props.children}>
      {(object, i) => {
        // eslint-disable-next-line solid/reactivity
        const o = sceneStore.objects.get(object) as EmbObject;
        if (!o) return null;
        const Component = SCENE_OBJECT_LOOKUP[o.type];
        return <Component {...o} order={i()} />;
      }}
    </For>
  );
};
