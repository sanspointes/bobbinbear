import { P } from "@bearbroidery/solixi";
import { Point } from "@pixi/core";
import { Container } from "@pixi/display";
import { Graphics } from "@pixi/graphics";
import { createEffect, createMemo, For, onMount, useContext } from "solid-js";
import { EmbVector } from ".";
import { useHoverSelectOutline } from "../../composables/useHoverSelectOutline";
import { mapTemporarySceneObjects } from "../../composables/useVirtualSceneObjects";
import { AppContext } from "../../store";
import { EmbState, SceneObjectChildren } from "..";
import { updateGraphicsWithSegments } from "./utils";
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
      updateGraphicsWithSegments(graphics, props.segments, props.fill);
    }
  });
  onMount(() => {
    if (!graphics) return;
    useHoverSelectOutline(graphics, props);
  });

  const { sceneStore } = useContext(AppContext);

  const isAppInspecting = createMemo(() => sceneStore.inspecting !== undefined);
  const isThisInspecting = createMemo(() =>
    isAppInspecting() && sceneStore.inspecting === props.id
  );

  const embSegments = mapTemporarySceneObjects(
    // eslint-disable-next-line solid/reactivity
    () => props.segments,
    // eslint-disable-next-line solid/reactivity
    (seg): EmbVecSeg => {
      console.log(seg);
      return ({
      id: seg.id,
      type: 'vec-seg',
      position: new Point(),
      parent: props.id,
      children: [],
      segment: seg,
      stroke: props.stroke,
    })
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
        {(uuid, i) => {
          // eslint-disable-next-line solid/reactivity
          const id = uuid();
          if (!id) return null;
          const o = sceneStore.objects.get(id) as EmbVecSeg & EmbState;
          if (!o) return null;
          console.log('Emb segment', id, o);
          return <EmbVecSegView {...o} order={i()} />
        }}
      </For>

      <SceneObjectChildren children={props.children} />
    </P.Container>
  );
};
