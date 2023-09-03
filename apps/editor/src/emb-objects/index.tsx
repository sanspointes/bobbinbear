import { For, useContext } from "solid-js";
import { EmbCanvas, EmbCanvasView } from "./canvas";
import { EmbGroup, EmbGroupView } from "./group";
import { EmbNode, EmbNodeView } from "./node";
import { EmbBase, EmbHasVirtual, EmbState } from "./shared";
import { EmbVector, EmbVectorView } from "./vector";
import { AppContext } from "../store";
import { EmbVecSeg, EmbVecSegView } from "./vec-seg";

export * from "./shared";

export * from "./node";
export * from "./vector";
export * from "./canvas";
export * from "./group";

export type EmbObject =
    & (
        | EmbVecSeg
        | EmbVector
        | EmbCanvas
        | EmbNode
        | EmbGroup
    )
    & EmbHasVirtual;

export type EmbObjectType = EmbObject["type"];

export type SceneObjectPropsLookup = {
    "canvas": EmbCanvas;
    "vector": EmbVector;
    "vec-seg": EmbVecSeg;
    "group": EmbGroup;
    "node": EmbNode;
};

const SCENE_OBJECT_LOOKUP = {
    "canvas": EmbCanvasView,
    "vector": EmbVectorView,
    "vec-seg": EmbVecSegView,
    "node": EmbNodeView,
    "group": EmbGroupView,
};

export const SceneObjectChildren = (
    props: Pick<EmbBase, "children">,
) => {
    const { sceneStore } = useContext(AppContext);
    return (
        <For each={props.children}>
            {(object, i) => {
                // eslint-disable-next-line solid/reactivity
                const o = sceneStore.objects.get(object) as EmbObject & EmbState;
                if (!o) return null;
                const Component = SCENE_OBJECT_LOOKUP[o.type];
                return <Component {...o} order={i()} />;
            }}
        </For>
    );
};
