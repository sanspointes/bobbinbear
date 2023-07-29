/*
 * UTILITY TYPES
 */
export type NonFunctionKeys<T> = {
  // eslint-disable-next-line @typescript-eslint/ban-types
  [K in keyof T]: T[K] extends Function ? never : K;
}[keyof T];
export type Overwrite<T, O> = Omit<T, NonFunctionKeys<O>> & O;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type Constructable = new (...args: any[]) => any;
export type Args<T> = T extends Constructable ? ConstructorParameters<T>
  : unknown[];

// eslint-disable-next-line @typescript-eslint/ban-types
export interface ClassType<T> extends Function {
  new (...args: unknown[]): T;
}

// Attach
export type AttachFnStrategy<
  TSource extends Constructable,
  TContext extends object,
> = (
  state: TContext,
  parent: SxiObject<Constructable, TContext>,
  child: SxiObject<TSource, TContext>,
) => () => void;
/**
 * Strategy for attaching/detatching a child to a parent.  Can either be a string, representing the function field on the parent
 * where the child is passed in as a parameter, or a method that provides access to both the parent and child.
 */
export type AttachStrategy<
  TSource extends Constructable,
  TContext extends object,
> = string | AttachFnStrategy<TSource, TContext>;

// INSTANCE TYPES
//

export type SxiInstanceReservedProps<
  TSource extends Constructable,
  TContext extends object,
  O extends SxiObject<TSource, TContext> = SxiObject<TSource, TContext>,
> = {
  args?: ConstructorParameters<TSource>;
  object?: O;
  visible?: boolean;
  attach?: AttachStrategy<TSource, TContext>;
};

export type SxiObjectMetadata<
  TSource extends Constructable,
  TContext extends object,
> = {
  __sxi: SxiInstance<TSource, TContext>;
};
export type SxiObject<
  TSource extends Constructable,
  TContext extends object,
  TObject extends InstanceType<TSource> = InstanceType<TSource>,
> =
  & TObject
  & SxiObjectMetadata<TSource, TContext>;

/**
 * Internal state for a SxiObject, stored under the object's `__sxi` iey.
 */
export type SxiInstance<
  TSource extends Constructable,
  TContext extends object,
> = {
  solixi: TContext;
  type: string;
  parent?: SxiInstance<Constructable, TContext>;
  object: SxiObject<TSource, TContext>;
  children: SxiInstance<Constructable, TContext>[];
  props: SxiInstanceReservedProps<TSource, TContext>;
};
