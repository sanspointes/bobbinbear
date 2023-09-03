import {
  Accessor,
  createMemo,
  mapArray,
  onCleanup,
  useContext,
} from "solid-js";
import { createStore } from "solid-js/store";
import { EmbBase, EmbState } from "../emb-objects/shared";
import { AppContext } from "../store";
import { access, MaybeAccessor } from "@solid-primitives/utils";
import { EMB_STATE_DEFAULTS } from "../emb-objects/shared";

export const useTemporarySceneObject = <TObject extends EmbBase>(
  obj: MaybeAccessor<(TObject & Partial<EmbState>)>,
): TObject & EmbState => {
  const { sceneStore } = useContext(AppContext);
  const [store, set] = createStore<TObject & EmbState>({ ...EMB_STATE_DEFAULTS ,...access(obj) });
  sceneStore.objects.set(store.id, store);
  // @ts-expect-error; Disregard
  sceneStore.objectSetters.set(store.id, set);

  onCleanup(() => {
    sceneStore.objects.delete(store.id);
    sceneStore.objectSetters.delete(store.id);
  });
  return store;
};

export const mapTemporarySceneObjects = <T, TObject extends EmbBase & Partial<EmbState>>(
  data: Accessor<false | readonly T[] | null | undefined>,
  mapFn: (v: T, i: Accessor<number>) => TObject,
) => {
  const v = mapArray(data, (v, i) => {
    const sceneObject = createMemo(() => {
      const sceneObject = mapFn(v, i);
      return useTemporarySceneObject(sceneObject);
    });
    return sceneObject;
  });
  return v;
};
