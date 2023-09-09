import { P } from '@bearbroidery/solixi';
import { Mesh } from '@pixi/mesh';
import { onMount } from 'solid-js';
import { useHoverSelectOutline } from '../../composables/useHoverSelectOutline';
import { EmbCanvas } from '.';
import { SceneObjectChildren } from '..';

type EmbCanvasProps = EmbCanvas & {
    order: number;
};
export const EmbCanvasView = (props: EmbCanvasProps) => {
    let mesh: Mesh | undefined;
    onMount(() => {
        if (!mesh) return;
        useHoverSelectOutline(mesh, props);
    });

    return (
        <P.Mesh
            id={props.id}
            soType={props.type}
            visible={props.visible}
            name={props.name}
            ref={mesh}
            scale={props.size}
            position={props.position}
            interactive={true}
            zIndex={props.order}
        >
            <P.PlaneGeometry args={[1, 1]} />
            <P.MeshMaterial tint={props.fill.color} />
            <SceneObjectChildren children={props.children} />
        </P.Mesh>
    );
};
