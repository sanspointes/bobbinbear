import { PMesh, PMeshMaterial, PPlaneGeometry } from "@bearbroidery/solixi";
import { CanvasSceneObject } from "../../store/scene";
import { SceneObjectChildren } from "./general";

export const GraphicSceneObjectView = (props: CanvasSceneObject) => ( 
  <PMesh scale={props.size} position={props.position}>
    <PPlaneGeometry args={[1, 1]} />
    <PMeshMaterial tint={props.backgroundColor} />
    <SceneObjectChildren children={props.children} />
  </PMesh>
);
