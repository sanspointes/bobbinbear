import { useContext } from "solid-js";
import { AppContext } from "../store";
import { P } from '@bearbroidery/solixi';

export function CursorTest() {
  const editor = useContext(AppContext);

  return <P.Mesh name="CursorTest" position={editor.inputStore.screenPosition}>
    <P.PlaneGeometry args={[10, 10]} />
    <P.MeshMaterial />
  </P.Mesh>
}
