import {
  Accessor,
  createMemo,
  createRenderEffect,
  mapArray,
  on,
  onCleanup,
  onMount,
  useContext,
} from "solid-js";
import { createStore } from "solid-js/store";
import { EmbBase } from "../types/scene";
import { AppContext } from "../store";
import { access, MaybeAccessor } from "@solid-primitives/utils";

export const useTemporarySceneObjects = (
  tempObjs: Accessor<(EmbBase | null)[]>,
) => {
  const ctx = useContext(AppContext)
  const { sceneStore } = ctx;
  createRenderEffect(on(tempObjs, (tempObjs, prevTempObjs) => {
    if (prevTempObjs) {
      for (const vNode of prevTempObjs) {
        if (!vNode) continue;
        const store = sceneStore.objects.get(vNode.id);
        if (store) {
          sceneStore.objects.delete(vNode.id);
        }
      }
    }
    if (tempObjs) {
      for (const vNode of tempObjs) {
        if (!vNode) continue;
        const [store, setStore] = createStore(vNode);
        sceneStore.objects.set(vNode.id, store);
        sceneStore.objectSetters.set(vNode.id, setStore);
      }
    }
  }));
};

export const useTemporarySceneObject = (
  obj: MaybeAccessor<(EmbBase)>,
) => {
  const { sceneStore } = useContext(AppContext);
  const [store, set] = createStore(access(obj));
  onMount(() => {
    sceneStore.objects.set(store.id, store);
    sceneStore.objectSetters.set(store.id, set);
  })

  onCleanup(() => {
    sceneStore.objects.delete(store.id);
    sceneStore.objectSetters.delete(store.id);
  });
};

export const mapTemporarySceneObjects = <T, TObject extends EmbBase>(
  data: Accessor<false | readonly T[] | null | undefined>,
  mapFn: (v: T, i: Accessor<number>) => TObject,
) => {
  const v = mapArray(data, (v, i) => {
    const sceneObject = createMemo(() => {
      const sceneObject = mapFn(v, () => i());
      if (sceneObject) useTemporarySceneObject(sceneObject);
      return sceneObject
    })
    return sceneObject;
  });
  return v;
};
