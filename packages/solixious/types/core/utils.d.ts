import { createEffect } from 'solid-js';
import * as display from '@pixi/display';
import * as core from '@pixi/core';
import type { Instance } from './proxy';
import type { Dpr, RootState } from './store';
import { Container } from 'pixi.js';
export type NonFunctionKeys<P> = {
    [K in keyof P]-?: P[K] extends Function ? never : K;
}[keyof P];
export type Overwrite<P, O> = Omit<P, NonFunctionKeys<O>> & O;
export type Properties<T> = Pick<T, NonFunctionKeys<T>>;
/**
 * An SSR-friendly useLayoutEffect.
 *
 * React currently throws a warning when using useLayoutEffect on the server.
 * To get around it, we can conditionally useEffect on the server (no-op) and
 * useLayoutEffect elsewhere.
 *
 * @see https://github.com/facebook/react/issues/14927
 */
export declare const useIsomorphicLayoutEffect: typeof createEffect;
export interface ObjectMap {
    nodes: {
        [name: string]: display.Container;
    };
    materials: {
        [name: string]: core.Shader;
    };
}
export declare function calculateDpr(dpr: Dpr): number;
/**
 * Returns instance root state
 */
export declare const getRootState: (obj: Instance['object']) => RootState | undefined;
export interface EquConfig {
    /** Compare arrays by reference equality a === b (default), or by shallow equality */
    arrays?: 'reference' | 'shallow';
    /** Compare objects by reference equality a === b (default), or by shallow equality */
    objects?: 'reference' | 'shallow';
    /** If true the keys in both a and b must match 1:1 (default), if false a's keys must intersect b's */
    strict?: boolean;
}
export declare const is: {
    obj: (a: any) => boolean;
    fun: (a: any) => a is Function;
    str: (a: any) => a is string;
    num: (a: any) => a is number;
    boo: (a: any) => a is boolean;
    und: (a: any) => boolean;
    arr: (a: any) => boolean;
    equ(a: any, b: any, { arrays, objects, strict }?: EquConfig): boolean;
};
export declare function buildGraph(object: Container): ObjectMap;
export interface Disposable {
    type?: string;
    clear?: () => void;
}
export declare function dispose<T extends Disposable>(obj: T): void;
export declare const INTERNAL_PROPS: string[];
export declare function getInstanceProps<T = any>(queue: any): Instance<T>['props'];
export declare function prepare<T = any>(target: T, root: RootState, type: string, props: Instance<T>['props']): Instance<T>;
export declare function resolve(root: any, key: string): {
    root: any;
    key: string;
    target: any;
};
export declare function attach(parent: Instance, child: Instance): void;
export declare function detach(parent: Instance, child: Instance): void;
export declare const RESERVED_PROPS: string[];
export declare const DEFAULTS: Map<any, any>;
export declare const applyProp: (object: Instance<Container>['object'], prop: string, value: any) => void;
export declare const applyProps: (object: Instance['object'], props: {
    [key: string]: any;
}) => void;
export declare function invalidateInstance(instance: Instance): void;
/**
 * Get a handle to the current global scope in window and worker contexts if able
 * https://github.com/pmndrs/react-three-fiber/pull/2493
 */
export declare const globalScope: false | typeof globalThis;
//# sourceMappingURL=utils.d.ts.map