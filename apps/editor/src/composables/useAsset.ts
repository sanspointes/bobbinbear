import { Assets } from '@pixi/assets';
import { Texture } from '@pixi/core';
import { MaybeAccessor, access } from '@solid-primitives/utils';
import { Accessor, createEffect, createMemo, createSignal, createUniqueId } from 'solid-js';

type UseAssetOptions<T> = {
  src: MaybeAccessor<string>,
  fallback: MaybeAccessor<T|undefined>,
}
export function useAsset<T>(opts: UseAssetOptions<T>): [Accessor<T|undefined>, Accessor<number>] {
  const [progress, setProgress] = createSignal(0);
  const [asset, setAsset] = createSignal<T>();

  const fallbackAsset = createMemo(() => {
    const a = asset();
    const fallback = access(opts.fallback);
    if (a) return a;
    if (fallback) return fallback;
    return undefined;
  })
  let loadPromise: Promise<void>|undefined;
  createEffect(() => {
    const src = access(opts.src);
    loadPromise = Assets.load(src, (v) => {
      setProgress(v)
    });
    loadPromise.then(v => {
      setAsset(() => v as unknown as T);
    })
  })

  return [fallbackAsset, progress];
} 

export function useTexture(opts: UseAssetOptions<Texture>) {
  return useAsset<Texture>(opts);
}
