import { PMesh, PPlaneGeometry, PMeshMaterial } from "@bearbroidery/solixi";
import { SceneObjectChildren } from "./general";
import { CanvasSceneObject } from "../types/scene";

export const CanvasSceneObjectView = (props: CanvasSceneObject) => ( 
  <PMesh scale={props.size} position={props.position} interactive={true}>
    <PPlaneGeometry args={[1, 1]} />
    <PMeshMaterial tint={props.backgroundColor} />
    <SceneObjectChildren children={props.children} />
  </PMesh>
);
