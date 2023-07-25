import * as THREE from "three";
import { StateSelector, EqualityChecker } from "zustand/vanilla";
import { RootState, RenderCallback } from "./core/store";
import { ObjectMap } from "./core/utils";
export interface Loader<T> extends THREE.Loader {
    load(url: string, onLoad?: (result: T) => void, onProgress?: (event: ProgressEvent) => void, onError?: (event: ErrorEvent) => void): unknown;
}
export type Extensions = (loader: THREE.Loader) => void;
export type LoaderResult<T> = T extends any[] ? Loader<T[number]> : Loader<T>;
export type ConditionalType<Child, Parent, Truthy, Falsy> = Child extends Parent ? Truthy : Falsy;
export type BranchingReturn<T, Parent, Coerced> = ConditionalType<T, Parent, Coerced, T>;
export declare function useStore(): import("zustand/vanilla").StoreApi<RootState>;
export declare function useThree<T = RootState, U = T>(selector?: StateSelector<RootState, U>, equalityFn?: EqualityChecker<U>): import("solid-js").Accessor<U>;
/**
 * Creates a signal that is updated when the given effect is run.
 *
 * @example
 * ```ts
 * const [count, setCount] = useSignal(0);
 * useFrame(() => {
 *  setCount(count + 1);
 * });
 * ```
 *
 * @param callback - a function to run on every frame render
 * @param renderPriority -  priority of the callback decides its order in the frameloop, higher is earlier
 */
export declare function useFrame(callback: RenderCallback, renderPriority?: number): void;
export declare function useGraph(object: THREE.Object3D): import("solid-js").Accessor<ObjectMap>;
export declare function loadingFn<T>(extensions?: Extensions, onProgress?: (event: ProgressEvent<EventTarget>) => void): (Proto: new () => LoaderResult<T>, ...input: string[]) => Promise<unknown[]>;
//# sourceMappingURL=hooks.d.ts.map