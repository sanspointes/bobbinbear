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
import { BaseSceneObject } from "../types/scene";
import { AppContext } from "../store";
import { access, MaybeAccessor } from "@solid-primitives/utils";
import { logger } from "../utils/logger";

export const useTemporarySceneObjects = (
  tempObjs: Accessor<(BaseSceneObject | null)[]>,
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
  obj: MaybeAccessor<(BaseSceneObject)>,
) => {
  const { sceneStore } = useContext(AppContext);
  const [store, set] = createStore(access(obj));
  logger.debug(`useTemporarySceneObject: Registering ${store.id}`);
  onMount(() => {
    sceneStore.objects.set(store.id, store);
    sceneStore.objectSetters.set(store.id, set);
  })

  onCleanup(() => {
    logger.debug(`useTemporarySceneObject: Unregistering ${store.id}`);
    sceneStore.objects.delete(store.id);
    sceneStore.objectSetters.delete(store.id);
  });
};

export const mapTemporarySceneObjects = <T, TObject extends BaseSceneObject>(
  data: Accessor<false | readonly T[] | null | undefined>,
  mapFn: (v: T, i: Accessor<number>) => TObject,
) => {
  const v = mapArray(data, (v, i) => {
    // @ts-expect-error ; Debug logging on expected
    logger.debug(`mapTemporarySceneObjects: Creating memo for ${v?.id ? v.id : v}`);
    const sceneObject = createMemo(() => {
      // @ts-expect-error ; Debug logging on expected
      logger.debug(`mapTemporarySceneObjects: Re-running memo for ${v?.id ? v.id : v}`);
      const sceneObject = mapFn(v, () => i());
      if (sceneObject) useTemporarySceneObject(sceneObject);
      return sceneObject
    })
    return sceneObject;
  });
  return v;
};
