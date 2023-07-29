import {
  createMemo,
  createRenderEffect,
  createEffect,
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
  object: SxiObject<TContext, TSource>,
  props: SxiInstance<TContext, TSource>["props"],
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
  TExtraProps extends Record<string, ExtraPropHandler<TContext, TSource>>,
>(
  target: InstanceType<TSource> & { __sxi?: SxiInstance<TContext, TSource> },
  state: TContext,
  type: string,
  props: ClassTypeProps2<TContext, TSource, TExtraProps>,
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  options: WrapConstructableOptions<TContext, TSource, TExtraProps>,
) => {
  console.debug(`CNST: Preparing ${type}`);
  const object: InstanceType<TSource> & { __sxi?: SxiInstance<TContext, TSource> } = target;

  const instance: SxiInstance<TContext, TSource> = object?.__sxi ?? {
    solixi: state,
    type,
    parent: null as unknown as SxiInstance<TContext, Constructable>,
    object: object as SxiObject<TContext, TSource>,
    children: [],
    props: getInstanceProps(props),
  };

  if (object) {
    object.__sxi = instance;
    if (type) {
      // applyProps(object, props, )

    }
  }


  if (props.ref) {
    props.ref(object);
  }
  return instance;
};

export type ExtraPropHandler<
  TContext extends object,
  TSource extends Constructable,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  V = any,
> = (
  ctx: TContext,
  parent: SxiObject<TContext, Constructable>,
  object: SxiObject<TContext, TSource>,
  value: V,
) => void | (() => void);

export type ExtraPropsHandlers<
  TContext extends object,
  TSource extends Constructable,
> = { [k: string]: ExtraPropHandler<TContext, TSource> };

type ExtraPropsSignature<TContext extends object, T extends ExtraPropsHandlers<TContext, Constructable>> = {
  [K in keyof T]: Parameters<T[K]>[3]
}
export type ClassTypeReservedProps<TContext extends object, TSource extends Constructable> = {
  ref?: InstanceType<TSource>|SxiObject<TContext, TSource>,
  args?: ConstructorParameters<TSource>,
  children?: JSX.Element | null
}


export type ClassTypeProps2<
  TContext extends object,
  TSource extends Constructable,
  TExtraProps extends ExtraPropsHandlers<TContext, TSource>,
> = Partial<Overwrite<
  Pick<InstanceType<TSource>, NonFunctionKeys<InstanceType<TSource>>>, // Set all fields on instance type.
  ExtraPropsSignature<TContext, TExtraProps> & ClassTypeReservedProps<TContext, TSource> // Overwride defaults with extra props + reserved props types.
>>;
  

export type WrapConstructableOptions<
  TContext extends object,
  TSource extends Constructable,
  TExtraProps extends Record<string, ExtraPropHandler<TContext, TSource>>,
> = {
  // How to attach this object to the parent
  attach: AttachStrategy<TContext, TSource>;
  // Default args incase args is emitted in props
  defaultArgs: ConstructorParameters<TSource>;
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
  TExtraProps extends Record<string, ExtraPropHandler<TContext, TSource>>,
  TObject extends InstanceType<TSource> = InstanceType<TSource>,
>(
  source: TSource,
  options: WrapConstructableOptions<TContext, TSource, TExtraProps>,
  useState: () => TContext,
) => {
  const Component = (
    props: ClassTypeProps2<TContext, TSource, TExtraProps>
  ) => {
    const state = useState();

    const object = createMemo(() => {
      const args: ConstructorParameters<TSource> = props.args ?? options.defaultArgs;

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
