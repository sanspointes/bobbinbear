import { P } from "@bearbroidery/solixi";
import { SceneObjectChildren } from "./general";
import { CanvasSceneObject } from "../types/scene";
import { Mesh } from "@pixi/mesh";
import { createRenderEffect, onMount } from "solid-js";
import { OutlineFilter } from "@pixi/filter-outline";
import { arrayRemoveEl } from "../utils/array";
import { metadata } from "../utils/metadata";

const HOVER_FILTER = new OutlineFilter(2, 0x0A8CE9, 0.1, 1);

export const CanvasSceneObjectView = (props: CanvasSceneObject) => {
  let mesh: Mesh|undefined;
  onMount(() => {
    if (!mesh) return;
    mesh.filters = [];
    metadata.set(mesh, {
      type: props.type,
      id: props.id,
    })
  })

  let outlinePushed = false;
  const outlineFilter = new OutlineFilter(3, 0x0A8CE9, 0.1, 1);
  createRenderEffect(() => {
    const needsPush = !outlinePushed && (props.hovered || props.selected);
    const needsRemove = outlinePushed && (!props.hovered && !props.selected);

    if (props.selected) outlineFilter.color = 0x41A3E9;
    else if (props.hovered) outlineFilter.color = 0x0A8CE9;

    if (!mesh) return;
    if (needsPush) {
      mesh.filters!.push(outlineFilter);
      outlinePushed = true;
    } else if (needsRemove) {
      arrayRemoveEl(mesh.filters!, outlineFilter);
      outlinePushed = false;
    } 
  })

  return ( 
  <P.Mesh ref={mesh} scale={props.size} position={props.position} interactive={true}>
    <P.PlaneGeometry args={[1, 1]} />
    <P.MeshMaterial tint={props.backgroundColor} />
    <SceneObjectChildren children={props.children} />
  </P.Mesh>
)
};
