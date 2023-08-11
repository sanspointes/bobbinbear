import { onMount, splitProps } from "solid-js";
import { Sprite } from "@pixi/sprite";
import { P } from "@bearbroidery/solixi";
import { GroupSceneObject } from "../types/scene";
import { SceneObjectChildren } from "./general";

type GroupSceneObjectViewProps = GroupSceneObject & {
  order: number;
};

export function GroupSceneObjectView(props: GroupSceneObjectViewProps) {
  return (
    <P.Container
      name={`${props.name}-${props.id}`}
      zIndex={props.order}
      visible={props.visible}
      position={props.position}
      interactive={false}
    >
      <SceneObjectChildren children={props.children} />
    </P.Container>
  );
}
