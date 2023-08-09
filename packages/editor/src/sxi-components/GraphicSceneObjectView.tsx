import { P } from "@bearbroidery/solixi";
import { SceneObjectChildren } from "./general";
import { GraphicSceneObject } from "../types/scene";

export const GraphicSceneObjectView = (props: GraphicSceneObject) => ( 
  <P.Mesh position={props.position} interactive={true}>
    <P.PlaneGeometry args={[1, 1]} />
    <P.MeshMaterial />
    <SceneObjectChildren children={props.children} />
  </P.Mesh>
);
