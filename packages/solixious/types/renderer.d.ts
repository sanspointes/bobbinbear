import { Accessor, Component, JSX } from "solid-js";
export declare const threeReconciler: {
    applyProps: typeof import("./core").applyProps;
    applyProp: typeof import("./core").applyProp;
    appendChild: (parentInstance: import("./core/renderer").Instance, child: import("./core/renderer").Instance) => void;
    createInstance: (type: string, { args, attach, ...props }: import("./core/renderer").InstanceProps, root: import("./core/renderer").Instance | import("zustand/vanilla").StoreApi<import(".").RootState>) => import("./core/renderer").Instance;
    switchInstance: (instance: import("./core/renderer").Instance, type: string, newProps: import("./core/renderer").InstanceProps) => void;
    insertBefore: (parentInstance: import("./core/renderer").Instance, child: import("./core/renderer").Instance, beforeChild: import("./core/renderer").Instance) => void;
    removeChild: (parentInstance: import("./core/renderer").Instance, child: import("./core/renderer").Instance, canDispose?: boolean | undefined) => void;
    removeRecursive: (array: import("./core/renderer").Instance[], parent: import("./core/renderer").Instance, dispose?: boolean) => void;
    attach: typeof import("./core").attach;
};
export declare const threeRenderer: import("solid-js/universal/types/universal").Renderer<import("./core/renderer").Instance>;
export declare const render: (code: () => import("./core/renderer").Instance, node: import("./core/renderer").Instance) => () => void, effect: <T>(fn: (prev?: T | undefined) => T, init?: T | undefined) => void, memo: <T>(fn: () => T, equal: boolean) => () => T, createComponent: <T>(Comp: (props: T) => import("./core/renderer").Instance, props: T) => import("./core/renderer").Instance, createElement: (tag: string) => import("./core/renderer").Instance, createTextNode: (value: string) => import("./core/renderer").Instance, insertNode: (parent: import("./core/renderer").Instance, node: import("./core/renderer").Instance, anchor?: import("./core/renderer").Instance | undefined) => void, insert: <T>(parent: any, accessor: T | (() => T), marker?: any) => import("./core/renderer").Instance, spread: <T>(node: any, accessor: T | (() => T), skipChildren?: Boolean | undefined) => void, setProp: <T>(node: import("./core/renderer").Instance, name: string, value: T, prev?: T | undefined) => T, mergeProps: (...sources: unknown[]) => unknown, use: <A, T>(fn: (element: import("./core/renderer").Instance, arg: A) => T, element: import("./core/renderer").Instance, arg: A) => T;
export * from "solid-js";
type DynamicProps<T> = T & {
    children?: any;
    component?: Component<T> | string | keyof JSX.IntrinsicElements;
};
/**
 * renders an arbitrary custom or native component and passes the other props
 * ```typescript
 * <Dynamic component={multiline() ? 'textarea' : 'input'} value={value()} />
 * ```
 * @description https://www.solidjs.com/docs/latest/api#%3Cdynamic%3E
 */
export declare function Dynamic<T>(props: DynamicProps<T>): Accessor<JSX.Element>;
//# sourceMappingURL=renderer.d.ts.map