import { PMesh, PPlaneGeometry, PMeshMaterial } from "@bearbroidery/solixi";
import { CanvasSceneObject } from "../../store/scene";
import { SceneObjectChildren } from "./general";

export const CanvasSceneObjectView = (props: CanvasSceneObject) => ( 
  <PMesh scale={props.size} position={props.position}>
    <PPlaneGeometry args={[1, 1]} />
    <PMeshMaterial tint={props.backgroundColor} />
    <SceneObjectChildren children={props.children} />
  </PMesh>
);
