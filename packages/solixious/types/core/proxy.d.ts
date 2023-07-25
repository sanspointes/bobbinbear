import { Accessor, Component, JSXElement } from 'solid-js';
import * as display from '@pixi/display';
import * as mesh from '@pixi/mesh';
import * as core from '@pixi/core';
import type { PixiElement } from '../three-types';
import { EventHandlers } from './events';
import { RootState } from './store';
export type AttachFnType<O = any> = (parent: any, self: O) => () => void;
export type AttachType<O = any> = string | AttachFnType<O>;
export type ConstructorRepresentation = new (...args: any[]) => any;
export interface Catalogue {
    [name: string]: ConstructorRepresentation;
}
export type Args<T> = T extends ConstructorRepresentation ? ConstructorParameters<T> : any[];
export interface InstanceProps<T = any, P = any> {
    args?: Args<P>;
    object?: T;
    visible?: boolean;
    dispose?: null;
    attach?: AttachType<T>;
}
export interface Instance<O = any> {
    root: RootState;
    type: string;
    parent: Instance | null;
    children: Instance[];
    props: InstanceProps<O> & Record<string, unknown>;
    object: O & {
        __r3f?: Instance<O>;
    };
    eventCount: number;
    handlers: Partial<EventHandlers>;
    attach?: AttachType<O>;
    previousAttach?: any;
    isHidden: boolean;
    autoRemovedBeforeAppend?: boolean;
}
export declare const catalogue: Catalogue;
export declare const extend: (objects: Partial<Catalogue>) => void;
export declare const ParentContext: import("solid-js").Context<(() => Instance) | undefined>;
export type Constructor<Instance = any> = {
    new (...args: any[]): Instance;
};
export type ThreeComponent<Source extends Constructor> = Component<PixiElement<Source>>;
type ThreeComponentProxy<Source> = {
    [K in keyof Source]: Source[K] extends Constructor ? ThreeComponent<Source[K]> : undefined;
};
export declare const createPixiComponent: <TSource extends Constructor<any>>(source: TSource) => ThreeComponent<TSource>;
export declare const parentChildren: <T extends display.Container<display.DisplayObject>>(getObject: Accessor<T & {
    __r3f?: Instance<T> | undefined;
}>, props: any) => void;
export declare function useObject(getObject: () => Instance['object'], props: any): void;
export declare function Primitive<T>(props: T & {
    object: T;
    children?: JSXElement;
    ref: T | ((value: T) => void);
}): Accessor<T & {
    __r3f?: Instance<T> | undefined;
}>;
export declare function createThreeComponentProxy<Source extends Record<string, any>>(source: Source): ThreeComponentProxy<Source>;
/**
 * The `solid-three` reactor. For every class exposed by `THREE`, this object contains a
 * `solid-three` component that wraps the class.
 */
export declare const T: ThreeComponentProxy<{
    Container: typeof display.Container;
    Mesh: typeof mesh.Mesh;
    MeshMaterial: typeof mesh.MeshMaterial;
    MeshGeometry: typeof mesh.MeshGeometry;
    PlaneGeometry: any;
    Shader: typeof core.Shader;
}>;
export {};
//# sourceMappingURL=proxy.d.ts.map