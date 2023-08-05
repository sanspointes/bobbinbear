import { PMesh, PMeshMaterial, PPlaneGeometry } from "@bearbroidery/solixi";
import { SceneObjectChildren } from "./general";
import { GraphicSceneObject } from "../types/scene";

export const GraphicSceneObjectView = (props: GraphicSceneObject) => ( 
  <PMesh position={props.position} interactive={true}>
    <PPlaneGeometry args={[1, 1]} />
    <PMeshMaterial />
    <SceneObjectChildren children={props.children} />
  </PMesh>
);
