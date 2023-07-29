import {
  createMemo,
  createRenderEffect,
  JSX,
  mapArray,
  splitProps,
} from "solid-js";
import {
  AttachStrategy,
  Constructable,
  NonFunctionKeys,

  Overwrite,

  SxiInstance,
  SxiObject,
} from "./types";

export const INTERNAL_PROPS = ["children", "ref"];

// Gets only instance props from proxy-component
export function getInstanceProps<T extends Constructable>(
  queue: Record<string, unknown>,
): SxiInstance<T>["props"] {
  // SOLID-THREE-NOTE:  solid-three has to use splitProps so getters are not resolved
  const [_, props] = splitProps(queue, INTERNAL_PROPS);
  return props;
}

// export const applyProp = <T extends Constructable>(
//   object: SxiObject<T>,
//   prop: string,
//   value: unknown,
// ) => {
//   const instance = object.__sxi as SxiInstance<T>;
// };

export const applyProps = <TSource extends Constructable, TContext extends object, >(
  object: SxiObject<TSource, TContext>,
  props: SxiInstance<TSource, TContext>["props"],
) =>
  createRenderEffect(mapArray(() => Object.keys(props), (key) => {
    /* We wrap it in an effect only if a prop is a getter or a function */
    // const descriptors = Object.getOwnPropertyDescriptor(props, key);
    // const isDynamic =
    //   !!(descriptors?.get || typeof descriptors?.value === "function");
    // const update = (value: unknown) => applyProp(object, key, value);
    // isDynamic
    //   ? createRenderEffect(on(() => props[key], update))
    //   : update(props[key]);
  }));

/**
 * Wraps an object in a SxiInstance<T>
 * @template T extends Constructable - Type of obj to wrap
 * @param target - Obj to wrap
 * @param state - Shared SxiState
 * @param type - Type string
 * @param props - Props
 * @returns
 */
const prepareObject = <
  TContext extends object,
  TSource extends Constructable,
  TExtraProps extends Record<string, ExtraPropHandler<TSource, TContext>>,
>(
  target: InstanceType<TSource> & { __sxi?: SxiInstance<TSource, TContext> },
  state: TContext,
  type: string,
  props: SxiInstance<TSource, TContext>["props"],
  options: ClassTypeProps<TSource, TExtraProps>,
) => {
  const object: InstanceType<TSource> & { __sxi?: SxiInstance<TSource, TContext> } = target;

  const instance: SxiInstance<TSource, TContext> = object?.__sxi ?? {
    solixi: state,
    type,
    parent: null as unknown as SxiInstance<Constructable, TContext>,
    object: object as SxiObject<TSource, TContext>,
    children: [],
    props: getInstanceProps(props),
  };

  if (object) {
    object.__sxi = instance;
    if (type) {
      applyProps(object, props, )

    }
  }
  return instance;
};

export type ExtraPropHandler<
  TSource extends Constructable,
  TContext extends object,
  V = unknown,
> = (
  parent: SxiObject<Constructable, TContext>,
  object: SxiObject<TSource, TContext>,
  value: V,
) => void | (() => void);

type ExtraPropsHandlers<
  TSource extends Constructable,
  TContext extends {},
> = { [k: string]: ExtraPropHandler<TSource, TContext> };

type ExtraPropsSignature<TContext extends {}, T extends ExtraPropsHandlers<Constructable, TContext>> = {
  [K in keyof T]: Parameters<T[K]>[3]
}
export type ClassTypeProps<
  TSource extends Constructable,
  TContext extends object,
  TExtraProps extends Record<string, ExtraPropHandler<TSource, TContext>> = Record<string, ExtraPropHandler<Constructable, TContext>>,
  TObject extends InstanceType<TSource> = InstanceType<TSource>,
> = 
  & { args?: ConstructorParameters<TSource>, children?: JSX.Element | JSX.Element[] } 
  & Partial<
      Overwrite<
        Pick<TObject, NonFunctionKeys<TObject>>, // All fields can be set
        ExtraPropsSignature<TExtraProps, TContext>
      >
    > 
;

export type WrapConstructableOptions<
  TSource extends Constructable,
  TContext extends object,
  TExtraProps extends Record<string, ExtraPropHandler<TSource, TContext>> = Record<string, ExtraPropHandler<TSource, TContext>>,
> = {
  // How to attach this object to the parent
  attach: AttachStrategy<TSource, TContext>;
  // Extra props and their handlers
  extraProps: TExtraProps;
};


/**
 * Wraps a Constructable class in a SolidJS component, to be used in JSX.
 *
 * @template TSource extends Constructable -
 * @param source - Class to wrap
 * @param options - Options defining how to attach parent -> children + add extra behaviours like
 * @returns
 */
export const wrapConstructable = <
  TContext extends object,
  TSource extends Constructable,
  TExtraProps extends Record<string, ExtraPropHandler<TSource, TContext>>,
  TObject extends InstanceType<TSource> = InstanceType<TSource>,
>(
  source: TSource,
  options: WrapConstructableOptions<TSource, TExtraProps>,
  useState: () => TContext,
) => {
  const Component = (
    props: ClassTypeProps<TSource, TExtraProps>
  ) => {
    const state = useState();

    const object = createMemo(() => {
      const args: ConstructorParameters<TSource> | unknown[] = props.args ?? [];

      const sourceObject: TObject = new source(args);
      const instance = prepareObject<TContext, TSource, TExtraProps>(
        sourceObject,
        state,
        source.name,
        props,
        options,
      );
      return instance.object;
    });

    return object as unknown as JSX.Element;
  };
  return Component;
};
