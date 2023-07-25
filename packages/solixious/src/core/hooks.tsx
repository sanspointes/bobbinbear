import {
  createMemo,
  createResource,
  onCleanup,
  untrack,
  useContext,
} from "solid-js";

import { Stages } from "./stages";
import { context } from "./store";
import { buildGraph } from "./utils";

import type { RenderCallback, StageTypes, UpdateCallback } from "./store";
import type { ObjectMap } from "./utils";
import { Container } from "pixi.js";

/**
 * Accesses R3F's internal state, containing renderer, canvas, scene, etc.
 * @see https://docs.pmnd.rs/react-three-fiber/api/hooks#usethree
 */
export function useThree() {
  const store = useContext(context);
  if (!store) {
    throw new Error("R3F: Hooks can only be used within the Canvas component!");
  }
  return store;
}

/**
 * Executes a callback before render in a shared frame loop.
 * Can order effects with render priority or manually render with a positive priority.
 * @see https://docs.pmnd.rs/react-three-fiber/api/hooks#useframe
 */

export function useFrame(
  callback: RenderCallback,
  renderPriority: number = 0,
): void {
  const store = useThree();
  const subscribe = store.internal.subscribe;
  const cleanup = subscribe(
    (state, delta, frame) => untrack(() => callback(state, delta, frame)),
    renderPriority,
    store,
  );

  onCleanup(cleanup);
}

/**
 * Executes a callback in a given update stage.
 * Uses the stage instance to indetify which stage to target in the lifecycle.
 */
export function useUpdate(
  callback: UpdateCallback,
  stage: StageTypes = Stages.Update,
) {
  const store = useThree();
  const stages = store.internal.stages;
  // Throw an error if a stage does not exist in the lifecycle
  if (!stages.includes(stage)) {
    throw new Error(`An invoked stage does not exist in the lifecycle.`);
  }
  // Subscribe on mount, unsubscribe on unmount
  const cleanup = stage.add(
    (state, delta, frame) => untrack(() => callback(state, delta, frame)),
    store,
  );
  onCleanup(cleanup);
}

/**
 * Returns a node graph of an object with named nodes & materials.
 * @see https://docs.pmnd.rs/react-three-fiber/api/hooks#usegraph
 */
export function useGraph(object: Container) {
  return createMemo(() => buildGraph(object));
}

// export interface Loader extends assets.Loader {
//   load(
//     assetsToLoadIn: string | string[] | assets.LoadAsset | assets.LoadAsset[],
//     onProgress: (progress: number) => void,
//   ):  Promise<{ [key: string]: any } | any>;
// }
//
// export type LoaderProto<T> = new (...args: any[]) => Loader<T>;
// export type LoaderResult<T> = T extends { scene: THREE.Object3D }
//   ? T & ObjectMap
//   : T;
// export type Extensions<T> = (loader: Loader<T>) => void;
//
// function loadingFn<T>(
//   extensions?: Extensions<T>,
//   onProgress?: (event: ProgressEvent) => void,
// ) {
//   return function (Proto: LoaderProto<T>, ...input: string[]) {
//     // Construct new loader and run extensions
//     const loader = new Proto();
//     if (extensions) extensions(loader);
//     // Go through the urls and load them
//     return Promise.all(
//       input.map(
//         (input) =>
//           new Promise<LoaderResult<T>>((res, reject) =>
//             loader.load(
//               input,
//               (data: any) =>
//                 res(
//                   data?.scene instanceof THREE.Object3D
//                     ? Object.assign(data, buildGraph(data.scene))
//                     : data,
//                 ),
//               onProgress,
//               (error) =>
//                 reject(new Error(`Could not load ${input}: ${error.message})`)),
//             )
//           ),
//       ),
//     );
//   };
// }
//
// const loaderCache = new Map();
//
// /**
//  * Synchronously loads and caches assets with a three loader.
//  */
// export function useLoader<T, U extends string | string[] | string[][]>(
//   Proto: LoaderProto<T>,
//   input: U,
//   extensions?: Extensions<T>,
//   onProgress?: (event: ProgressEvent) => void,
// ) {
//   // Use createResource to load async assets
//   const keys = (Array.isArray(input) ? input : [input]) as string[];
//
//   return createResource(
//     () => [Proto, ...keys] as const,
//     async ([Proto, ...keys]) => {
//       if (loaderCache.has([Proto.name, ...keys].join("-"))) {
//         return loaderCache.get([Proto.name, ...keys].join("-"));
//       }
//       const data = await loadingFn(extensions, onProgress)(
//         Proto as any,
//         ...(keys as any),
//       );
//       loaderCache.set(
//         [Proto.name, ...keys].join("-"),
//         Array.isArray(input) ? data : data[0],
//       );
//       if (Array.isArray(input)) return data;
//       return data[0];
//     },
//   )[0];
// }
//
// /**
//  * Preloads an asset into cache as a side-effect.
//  */
// useLoader.preload = function <T, U extends string | string[]>(
//   Proto: new () => LoaderResult<T>,
//   input: U,
//   extensions?: Extensions,
// ) {
//   const keys = (Array.isArray(input) ? input : [input]) as string[]
//   return preload(loadingFn<T>(extensions), [Proto, ...keys])
// }

/**
 * Removes a loaded asset from cache.
 */
// useLoader.clear = function <T, U extends string | string[]>(
//   Proto: new () => LoaderResult<T>,
//   input: U,
// ) {
//   const keys = (Array.isArray(input) ? input : [input]) as string[];
//   return loaderCache.delete([Proto.name, ...keys].join("-"));
// };
