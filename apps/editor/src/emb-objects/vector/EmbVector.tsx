import { P } from "@bearbroidery/solixi";
import { Point } from "@pixi/core";
import { Container } from "@pixi/display";
import { Graphics } from "@pixi/graphics";
import { createEffect, createMemo, For, onMount, useContext } from "solid-js";
import { EmbVector } from ".";
import { mapTemporarySceneObjects } from "../../composables/useVirtualSceneObjects";
import { AppContext } from "../../store";
import { EmbState, SceneObjectChildren } from "..";
import { drawVectorShapeToGraphic } from "./utils";
import { EmbVecSeg } from "../vec-seg";
import { EmbVecSegView } from "../vec-seg/EmbVecSeg";

type EmbVectorProps = EmbVector & EmbState & {
    order: number;
};
export const EmbVectorView = (props: EmbVectorProps) => {
    let container: Container | undefined;
    let graphics: Graphics | undefined;
    createEffect(() => {
        if (graphics) {
            drawVectorShapeToGraphic(graphics, props.shape, props.fill, props.line);
        }
    });
    onMount(() => {
        if (!graphics) return;
    })

    const { sceneStore } = useContext(AppContext);

    const isAppInspecting = createMemo(() => sceneStore.inspecting !== undefined);
    const isThisInspecting = createMemo(() =>
        isAppInspecting() && sceneStore.inspecting === props.id
    );

    const embSegments = mapTemporarySceneObjects(
        // eslint-disable-next-line solid/reactivity
        () => props.shape,
        // eslint-disable-next-line solid/reactivity
        (seg) => {
            const model: EmbVecSeg & Partial<EmbState> = {
                id: seg.id,
                type: "vec-seg",
                position: new Point(),
                parent: props.id,
                relatesTo: props.id,
                children: [],
                segment: seg,
                line: props.line,
                disableMove: true,
                inspecting: isThisInspecting(),
            }
            if (!isThisInspecting()) model.hovered = props.hovered
            if (!isThisInspecting()) model.selected = props.selected
            return model;
        },
    );

    return (
        <P.Container
            id={props.id}
            ref={container}
            zIndex={sceneStore.inspecting === props.id ? 500 : props.order}
            position={props.position}
        >
            <P.Graphics
                id={props.id}
                soType={props.type}
                name={`${props.id} ${props.name}`}
                visible={props.visible}
                ref={graphics}
                interactive={!isAppInspecting() || isThisInspecting()}
                alpha={!isAppInspecting() || isThisInspecting() ? 1 : 0.5}
            />

            <For each={embSegments()}>
                {(props, i) => {
                    // eslint-disable-next-line solid/reactivity
                    return <EmbVecSegView {...props()} order={i()} />;
                }}
            </For>

            <SceneObjectChildren children={props.children} />
        </P.Container>
    );
};
