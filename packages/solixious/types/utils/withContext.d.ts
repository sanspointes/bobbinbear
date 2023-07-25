import { Accessor, Context, JSX } from 'solid-js';
export type ContextProviderProps = {
    children?: JSX.Element;
} & Record<string, unknown>;
export type ContextProvider<T extends ContextProviderProps> = (props: {
    children: JSX.Element;
} & T) => JSX.Element;
/**
 * A utility-function to provide context to components.
 *
 * @param children Accessor of Children
 * @param context Context<T>
 * @param value T
 *
 * @example
 * ```tsx
 * const NumberContext = createContext<number>
 *
 * const children = withContext(
 *    () => props.children,
 *    NumberContext,
 *    1
 * )
 * ```
 */
export declare function withContext<T>(children: Accessor<JSX.Element | JSX.Element[]>, context: Context<T>, value: T): () => JSX.Element | JSX.Element[];
/**
 * A utility-function to provide multiple context to components.
 *
 * @param children Accessor of Children
 * @param values Array of tuples of `[Context<T>, value T]`.
 *
 * @example
 * ```tsx
 * const NumberContext = createContext<number>
 * const StringContext = createContext<string>
 * const children = withContext(
 *    () => props.children,
 *    [
 *      [NumberContext, 1],
 *      [StringContext, "string"]
 *    ]
 * )
 * ```
 */
export declare function withMultiContexts<T extends readonly [unknown?, ...unknown[]]>(children: Accessor<JSX.Element | JSX.Element[]>, values: {
    [K in keyof T]: readonly [Context<T[K]>, [T[K]][T extends unknown ? 0 : never]];
}): () => JSX.Element | JSX.Element[];
//# sourceMappingURL=withContext.d.ts.map